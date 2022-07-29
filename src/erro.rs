// TODO: delete this once constructed an Unauthorized Error
#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use http_api_problem::HttpApiProblem;
use sqlx::error::DatabaseError;

/// A type to be used for listing errors during request processing.
#[derive(Debug)]
pub struct ErrorMap<K, V>(HashMap<K, Vec<V>>);

impl<K, V> ErrorMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add a new error to this error map.
    pub fn add_error<T, U>(&mut self, key: T, value: U) -> &mut Self
    where
        T: Into<K> + Clone,
        U: Into<V>,
    {
        let key = key.into();

        if !self.0.contains_key(&key) {
            self.0.insert(key.clone(), Vec::new());
        }

        self.0.get_mut(&key).unwrap().push(value.into());
        self
    }

    /// Merge into this error map the provided error map.
    pub fn merge<T, U>(&mut self, other: ErrorMap<T, U>) -> &mut Self
    where
        T: Into<K> + Clone,
        U: Into<V>,
    {
        for (k, v) in other.0 {
            for err in v {
                self.add_error(k.clone().into(), err.into());
            }
        }

        self
    }
}

impl<K, V> Default for ErrorMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Deref for ErrorMap<K, V> {
    type Target = HashMap<K, Vec<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// A common error type that can be used throughout the API.
///
/// Can be returned in a `Result` from an API handler function.
///
/// For convenience, this represents both API errors as well as internal recoverable errors,
/// and maps them to appropriate status codes along with at least a minimally useful error
/// message in a JSON body.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    /// Return `401 Unauthorized`
    #[error("Autenticación Requerida")]
    Unauthorized,

    // /// Return `403 Forbidden`
    // #[error("user may not perform that action")]
    // Forbidden,

    // /// Return `404 Not Found`
    // #[error("request path not found")]
    // NotFound,
    /// Return `422 Unprocessable Entity`
    #[error("error in the request body")]
    UnprocessableEntity(ErrorMap<String, String>),

    /// Automatically return `500 Internal Server Error` on a `sqlx::Error`.
    ///
    /// Via the generated `From<sqlx::Error> for Error` impl,
    /// this allows using `?` on database calls in handler functions without a manual mapping step.
    ///
    /// The actual error message isn't returned to the client for security reasons.
    /// It should be logged instead.
    ///
    /// Note that this could also contain database constraint errors, which should usually
    /// be transformed into client errors (e.g. `422 Unprocessable Entity` or `409 Conflict`).
    /// See `ResultExt` below for a convenient way to do this.
    #[error("Un error ocurrió con la base de datos.")]
    Sqlx(#[from] sqlx::Error),

    /// Return `500 Internal Server Error` on a `eyre::Report`.
    ///
    /// `eyre::Report` is used in a few places to capture context and backtraces
    /// on unrecoverable (but technically non-fatal) errors which could be highly useful for
    /// debugging. We use it a lot in our code for background tasks or making API calls
    /// to external services so we can use `.wrap_err()` to refine the logged error.
    ///
    /// Via the generated `From<eyre::Report> for Error` impl, this allows the
    /// use of `?` in handler functions to automatically convert `eyre::Report` into a response.
    ///
    /// Like with `Error::Sqlx`, the actual error message is not returned to the client
    /// for security reasons.
    // TODO: show how to report error to developers
    #[error("Un error interno ocurrió en el servidor.")]
    Eyre(#[from] eyre::Report),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            // Self::Forbidden => StatusCode::FORBIDDEN,
            // Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Eyre(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // TODO: test this error structure
        let mut details_7807 = HttpApiProblem::with_title(status)
            // TODO: self hosted page explaining the error
            .type_url(format!("https://httpstatuses.io/{}", status.as_u16()))
            .detail(self.to_string())
            .value("timestamp", &time::OffsetDateTime::now_utc());

        match self {
            AppError::Unauthorized => {
                // Include the `WWW-Authenticate` challenge required in the specification
                // for the `401 Unauthorized` response code:
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                let mut hyper_response = details_7807.to_hyper_response();
                hyper_response.headers_mut().append(header::WWW_AUTHENTICATE, HeaderValue::from_static("Token"));
                return hyper_response.into_response();
            }
            // add errors to response
            AppError::UnprocessableEntity(errors_map) => {
                errors_map
                    .iter()
                    .for_each(|(key, errors)| {
                        details_7807.set_value(key, errors);
                    });
            },
            AppError::Sqlx(ref error) => {
                tracing::error!(?error, "SQLx error");
            }
            AppError::Eyre(ref error) => {
                tracing::error!(?error, "generic error");
            }
            // handle normally
            // AppError::Forbidden | AppError::NotFound => (),
        };

        details_7807.to_hyper_response().into_response()
    }
}

/// A little helper trait for more easily converting database constraint errors into API errors.
///
/// ```rust,ignore
/// let user_id = sqlx::query_scalar!(
///     r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
///     username,
///     email,
///     password_hash
/// )
///     .fetch_one(&ctxt.db)
///     .await
///     .on_constraint("user_username_key", |_| Error::unprocessable_entity([("username", "already taken")]))?;
/// ```
///
/// Something like this would ideally live in a `sqlx-axum` crate if it made sense to author one,
/// however its definition is tied pretty intimately to the `Error` type, which is itself
/// tied directly to application semantics.
///
/// To actually make this work in a generic context would make it quite a bit more complex,
/// as you'd need an intermediate error type to represent either a mapped or an unmapped error,
/// and even then it's not clear how to handle `?` in the unmapped case without more boilerplate.
pub trait ResultExt<T> {
    /// If `self` contains a `DatabaseError` constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> AppError,
    ) -> Result<T, AppError> {
        self.map_err(|e| match e.into() {
            AppError::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}

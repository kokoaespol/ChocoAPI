use axum::http::StatusCode;

#[allow(clippy::unused_async)]
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

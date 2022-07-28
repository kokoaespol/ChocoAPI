use crate::{
    configuration::{DatabaseSettings, Settings},
    repositories::{EmailRepository, ImageRepository, UserRepository},
    routes::{health_check, register},
};
use axum::{
    routing::{get, post, IntoMakeService},
    Extension, Router, Server,
};
use eyre::{Result, WrapErr};
use hyper::server::conn::AddrIncoming;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub struct Application {
    local_address: SocketAddr,
    server: Server<AddrIncoming, IntoMakeService<Router>>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self> {
        let connection_pool = get_connection_pool(&configuration.database).await?;
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        let address = SocketAddr::from((
            configuration.application.host,
            configuration.application.port,
        ));

        let app = app(connection_pool);

        let server = axum::Server::bind(&address).serve(app.into_make_service());

        let local_address = server.local_addr();

        Ok(Application {
            local_address,
            server,
        })
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.wrap_err("error running HTTP server")
    }

    pub fn local_address(&self) -> SocketAddr {
        self.local_address
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool> {
    // TODO: Add max_retries to the configuration file.
    const MAX_RETRIES: i32 = 5;

    let mut retries = 0;

    loop {
        match PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // We should leave some connections available for manual access.
            //
            // NOTE: If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(50)
            .connect_with(configuration.with_db())
            .await
            .wrap_err("error starting Postgres db")
        {
            Ok(pool) => break Ok(pool),
            Err(e) => {
                if retries < MAX_RETRIES {
                    tracing::warn!("retrying db connection");
                    retries += 1;
                } else {
                    break Err(e);
                }
            }
        }
    }
}

// TODO: only `merge` here and delegate to routes folder
#[must_use]
fn app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/register", post(register))
        .layer(Extension(UserRepository::new(db_pool.clone())))
        .layer(Extension(ImageRepository::new(db_pool.clone())))
        .layer(Extension(EmailRepository::new(db_pool.clone())))
        .layer(TraceLayer::new_for_http())
}

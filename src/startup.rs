use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::health_check,
};
use async_recursion::async_recursion;
use axum::{
    routing::{get, IntoMakeService},
    Extension, Router, Server,
};
use eyre::{Result, WrapErr};
use hyper::server::conn::AddrIncoming;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub struct Application {
    port: u16,
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

        let port = server.local_addr().port();

        Ok(Application { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<()> {
        self.server.await.wrap_err("error running HTTP server")
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool> {
    // If rust had an Alternative typeclass I could just "choose" between the different
    #[async_recursion]
    async fn try_connect(
        configuration: &DatabaseSettings,
        retries: i32,
        max_retries: i32,
    ) -> Result<PgPool> {
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
            pool @ Ok(_) => pool,
            Err(_) if retries < max_retries => {
                try_connect(configuration, retries + 1, max_retries).await
            }
            err @ Err(_) => err,
        }
    }

    // TODO: Add max_retries to the configuration file.
    let max_retries = 5;
    try_connect(configuration, 0, max_retries).await
}

// TODO: only `merge` here and delegate to routes folder
#[must_use]
fn app(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .layer(Extension(db_pool))
        .layer(TraceLayer::new_for_http())
}

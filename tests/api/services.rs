use std::ops::Deref;

use sqlx::{Connection, Executor, PgConnection, PgPool};

use super::wrappers::TestConfiguration;

/// A database to be used for testing.
pub struct TestDatabase(PgPool);

impl TestDatabase {
    /// Create a new TestDatabase from the provided DatabaseSettings.
    ///
    /// Creating a test database implies:
    /// 1. Creating a new database in Postgres with a random UUIDv4 as its name.
    /// 2. Running the migrations against the newly created database.
    pub async fn new(config: &TestConfiguration) -> Self {
        // Acquire a connection to Postgres
        let mut connection = PgConnection::connect_with(&config.without_db())
            .await
            .expect("failed to connect to Postgres");

        // Create database
        connection
            .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.db_name()))
            .await
            .expect("failed to create database");

        // Migrate database
        let connection_pool = PgPool::connect_with(config.with_db())
            .await
            .expect("failed to connect to Postgres");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("failed to migrate the database");

        TestDatabase(connection_pool)
    }
}

impl Deref for TestDatabase {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Se puede implementar Drop para que haga DROP DATABASE {database.name}

// TODO: Implementar servicio de Redis

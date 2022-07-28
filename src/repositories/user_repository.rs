use eyre::Context;
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::{
    erro::AppError,
    models::{InsertableUser, User},
};

/// A repository for managing users.
#[derive(Clone)]
pub struct UserRepository(PgPool);

impl UserRepository {
    /// Create a new `UserRepository` that works over the provided database connection.
    pub fn new(pool: PgPool) -> Self {
        UserRepository(pool)
    }

    /// Create a new user in the database.
    pub async fn create_user(&self, user: InsertableUser) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, full_name, profile_pic_id, email_id, passwd_hash)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            user.username(),
            user.full_name(),
            user.profile_pic_id(),
            user.email_id(),
            user.passwd_hash()
        )
        .fetch_one(&self.0)
        .await
        .map_err(AppError::Sqlx)
    }

    /// Get a single `User` by its id.
    pub async fn get_by_id(&self, id: Uuid) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.0)
            .await
            .wrap_err("failed to fetch user from database")
            .unwrap()
    }
}

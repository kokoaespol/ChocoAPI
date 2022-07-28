use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::erro::AppError;

#[derive(Clone)]
pub struct EmailRepository(PgPool);

impl EmailRepository {
    pub fn new(pool: PgPool) -> Self {
        EmailRepository(pool)
    }

    /// Create a new email in the database.
    pub async fn create_email(&self, email: String) -> Result<Uuid, AppError> {
        sqlx::query!(
            r#"
            INSERT INTO emails (email)
            VALUES ($1)
            RETURNING id
            "#,
            email
        )
        .fetch_one(&self.0)
        .await
        .map(|record| record.id)
        .map_err(AppError::Sqlx)
    }
}

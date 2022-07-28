use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct EmailRepository(PgPool);

impl EmailRepository {
    pub fn new(pool: PgPool) -> Self {
        EmailRepository(pool)
    }

    /// Create a new email in the database.
    pub fn create_email(&self) -> Uuid {
        todo!("not implemented")
    }
}

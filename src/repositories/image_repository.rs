use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct ImageRepository(PgPool);

impl ImageRepository {
    pub fn new(pool: PgPool) -> Self {
        ImageRepository(pool)
    }

    /// Create a new image in the database.
    pub fn create_image(&self) -> Uuid {
        todo!("not implemented")
    }
}

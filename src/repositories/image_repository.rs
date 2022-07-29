use std::io::Cursor;

use eyre::Context;
use image::io::Reader as ImageReader;
use sqlx::postgres::PgPool;
use std::path::PathBuf;
use uuid::Uuid;

use crate::erro::AppError;

#[derive(Clone)]
pub struct ImageRepository(PgPool);

impl ImageRepository {
    pub fn new(pool: PgPool) -> Self {
        ImageRepository(pool)
    }

    /// Create a new image mime type in the database.
    pub async fn create_image_mime_type(&self, mime_type: &str) -> i16 {
        sqlx::query!(
            r#"
            INSERT INTO image_mime_types (mime)
            VALUES ($1)
            RETURNING id
            "#,
            mime_type
        )
        .fetch_one(&self.0)
        .await
        .unwrap()
        .id
    }

    /// Create a new image in the database.
    pub async fn create_image(&self, mime_type: &str, bytes: Vec<u8>) -> Result<Uuid, AppError> {
        let img = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .wrap_err("failed to guess image format")?
            .decode()
            .wrap_err("failed to decode image")?;

        let width: i32 = img.width().try_into().unwrap();
        let height: i32 = img.height().try_into().unwrap();
        let image_id = Uuid::new_v4();
        let img_size: i32 = bytes.len().try_into().unwrap();

        // TODO: Check that the directory exists and all that.
        let file_path: PathBuf = ["image_uuid", &format!("image_file_{image_id}")]
            .iter()
            .collect();

        // Try to fetch an existing mime_type id or create a new one if it does not
        // already exist.
        let mime_id = match sqlx::query!(
            r#"SELECT id FROM image_mime_types WHERE mime = $1"#,
            mime_type
        )
        .fetch_optional(&self.0)
        .await
        .wrap_err("failed to fetch image mime type")?
        .map(|r| r.id)
        {
            Some(id) => id,
            None => self.create_image_mime_type(mime_type).await,
        };

        let id = sqlx::query!(r#"
                     INSERT INTO image_files (id, width_px, height_px, file_path, size_bytes, mime_id)
                     VALUES ($1, $2, $3, $4, $5, $6)
                     RETURNING id
                     "#,
                     image_id, width, height, file_path.to_str(), img_size, mime_id)
            .fetch_one(&self.0)
            .await
            .wrap_err("failed to insert image file in database")?
            .id;

        Ok(id)
    }
}

use axum::{extract::Multipart, http::StatusCode, Extension};
use eyre::{Context, ContextCompat};
use tracing::warn;

use crate::{
    erro::AppError,
    models::InsertableUserBuilder,
    repositories::{EmailRepository, ImageRepository, UserRepository},
};

/// Register a user.
pub async fn register(
    mut body: Multipart,
    Extension(user_repository): Extension<UserRepository>,
    Extension(email_repository): Extension<EmailRepository>,
    Extension(image_repository): Extension<ImageRepository>,
) -> Result<StatusCode, AppError> {
    let mut builder = InsertableUserBuilder::new();

    while let Some(field) = body
        .next_field()
        .await
        .wrap_err("failed to parse multipart form data")?
    {
        if let Some(field_name) = field.name() {
            match field_name {
                "username" => {
                    builder = builder.with_username(
                        field
                            .text()
                            .await
                            .wrap_err("failed to parse form username")?,
                    );
                }
                "password" => {
                    builder = builder.with_password(
                        field
                            .text()
                            .await
                            .wrap_err("failed to parse form password")?,
                    );
                }
                "full_name" => {
                    builder = builder.with_full_name(
                        field
                            .text()
                            .await
                            .wrap_err("failed to parse form full name")?,
                    );
                }
                "email" => {
                    let email = field.text().await.wrap_err("failed to parse form email")?;
                    builder = builder.with_email_id(email_repository.create_email(email).await?);
                }
                "profile_pic" => {
                    let mime_type = field
                        .content_type()
                        .wrap_err("failed to fetch image content type")?
                        .to_string();
                    let image = field.bytes().await.wrap_err("failed to parse form image")?;
                    builder = builder.with_profile_pic_id(
                        image_repository
                            .create_image(&mime_type, image.to_vec())
                            .await?,
                    );
                }
                _ => warn!("invalid field name in registration form: {}", field_name),
            }
        }
    }

    // TODO: insert permissions for new user
    // TODO: send confirmation email

    if let Some(insertable_user) = builder.build() {
        // TODO: Return this in the response body
        user_repository.create_user(insertable_user).await?;
        Ok(StatusCode::CREATED)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}

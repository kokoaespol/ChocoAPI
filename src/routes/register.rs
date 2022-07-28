use axum::{extract::Multipart, http::StatusCode, Extension};
use eyre::Context;
use tracing::warn;
use uuid::Uuid;

use crate::{
    erro::AppError,
    repositories::{EmailRepository, ImageRepository, UserRepository},
};

pub async fn register(
    mut body: Multipart,
    Extension(user_repository): Extension<UserRepository>,
    Extension(email_repository): Extension<EmailRepository>,
    Extension(image_repository): Extension<ImageRepository>,
) -> Result<StatusCode, AppError> {
    let mut username: String = Default::default();
    let mut password: String = Default::default();
    let mut full_name: Option<String> = None;
    let mut profile_pic_id: Option<Uuid> = None;
    let mut email_id: Uuid = Default::default();

    while let Some(field) = body
        .next_field()
        .await
        .wrap_err("failed to parse multipart form data")?
    {
        if let Some(field_name) = field.name() {
            // TODO: simplify this with a macro
            match field_name {
                "username" => {
                    username = field
                        .text()
                        .await
                        .wrap_err("failed to parse form username")?;
                }
                "password" => {
                    password = field
                        .text()
                        .await
                        .wrap_err("failed to parse form password")?;
                }
                "full_name" => {
                    full_name = Some(
                        field
                            .text()
                            .await
                            .wrap_err("failed to parse form full name")?,
                    );
                }
                "email" => {
                    let email = field.text().await.wrap_err("failed to parse form email")?;
                    email_id = email_repository.create_email();
                }
                "profile_pic" => {
                    let image = field.bytes().await.wrap_err("failed to parse form image")?;
                    profile_pic_id = Some(image_repository.create_image());
                }
                name => warn!("invalid field name in registration form {}", name),
            }
        } else {
            warn!("field is missing value in form data");
        }
    }

    if username != "" || password != "" || email_id == Uuid::default() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let _user = user_repository
        .create_user(username, password, full_name, profile_pic_id, email_id)
        .await?;

    Ok(StatusCode::CREATED)
}

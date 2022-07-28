use time::OffsetDateTime;
use uuid::Uuid;

/// A domain user.
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub full_name: Option<String>,
    pub profile_pic_id: Option<Uuid>,
    pub email_id: Uuid,
    pub passwd_hash: String,
    pub active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

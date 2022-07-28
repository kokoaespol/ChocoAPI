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

/// Represents a user to be inserted in the database.
pub struct InsertableUser {
    username: String,
    full_name: Option<String>,
    profile_pic_id: Option<Uuid>,
    email_id: Uuid,
    passwd_hash: String,
}

impl InsertableUser {
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn full_name(&self) -> Option<String> {
        self.full_name.clone()
    }

    pub fn profile_pic_id(&self) -> Option<Uuid> {
        self.profile_pic_id
    }

    pub fn email_id(&self) -> Uuid {
        self.email_id
    }

    pub fn passwd_hash(&self) -> String {
        self.passwd_hash.clone()
    }
}

/// Build a new `InsertableUser`.
pub struct InsertableUserBuilder {
    username: String,
    full_name: Option<String>,
    profile_pic_id: Option<Uuid>,
    email_id: Uuid,
    passwd_hash: String,
}

impl InsertableUserBuilder {
    pub fn new() -> Self {
        InsertableUserBuilder {
            username: String::default(),
            full_name: None,
            profile_pic_id: None,
            email_id: Uuid::default(),
            passwd_hash: String::default(),
        }
    }

    pub fn with_username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    pub fn with_full_name(mut self, full_name: String) -> Self {
        self.full_name = Some(full_name);
        self
    }

    pub fn with_password(mut self, passwd: String) -> Self {
        // TODO: hash password
        self.passwd_hash = passwd;
        self
    }

    pub fn with_email_id(mut self, email_id: Uuid) -> Self {
        self.email_id = email_id;
        self
    }

    pub fn with_profile_pic_id(mut self, profile_pic_id: Uuid) -> Self {
        self.profile_pic_id = profile_pic_id.into();
        self
    }

    pub fn build(self) -> Option<InsertableUser> {
        if self.username.is_empty()
            || self.passwd_hash.is_empty()
            || self.email_id == Uuid::default()
        {
            None
        } else {
            InsertableUser {
                username: self.username,
                full_name: self.full_name,
                profile_pic_id: self.profile_pic_id,
                email_id: self.email_id,
                passwd_hash: self.passwd_hash,
            }
            .into()
        }
    }
}

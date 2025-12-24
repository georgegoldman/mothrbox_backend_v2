// src/models/mod.rs
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id_string(&self) -> String {
        self.id.as_ref().map(|oid| oid.to_hex()).unwrap_or_default()
    }
}

// DTO for public user representation (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id_string(),
            email: user.email,
            created_at: user.created_at,
        }
    }
}

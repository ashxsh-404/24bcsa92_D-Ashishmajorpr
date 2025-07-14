use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

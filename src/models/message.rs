use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MessageWithSender {
    pub content: String,
    pub sender_name: String,
    pub sender_email: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageInput {
    pub content: String,
}

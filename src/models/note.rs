use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Note {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>, // ✅ Fix: make it optional
}


#[derive(Debug, Deserialize)]
pub struct CreateNoteInput {
    pub ticket_id: Uuid,
    pub content: String,
}

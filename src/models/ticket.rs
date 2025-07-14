use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicketInput {
    pub title: String,
    pub description: String,
    pub priority: String, // e.g., "low", "medium", "high"
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketInput {
    pub status: String, // e.g., "open", "in_progress", "resolved"
}

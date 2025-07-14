use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Article {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleInput {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct KnowledgeBaseQueryParams {
    pub search: Option<String>,
    pub tag: Option<String>,
}

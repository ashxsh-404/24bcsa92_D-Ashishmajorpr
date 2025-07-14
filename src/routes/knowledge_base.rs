use axum::{Router, routing::{post, get}};
use crate::handlers::knowledge_base_handler::{create_article_handler, get_all_articles_handler};
use sqlx::PgPool;

pub fn knowledge_base_routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_article_handler).get(get_all_articles_handler))
}

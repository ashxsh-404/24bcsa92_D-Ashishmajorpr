use axum::{Router, routing::{post, get}};
use crate::handlers::collaboration_handler::{post_message_handler, get_messages_handler};
use sqlx::PgPool;

pub fn collaboration_routes() -> Router<PgPool> {
    Router::new()
        .route("/:ticket_id", post(post_message_handler).get(get_messages_handler))
}

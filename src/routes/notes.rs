use axum::{
    Router,
    routing::{post, get},
};
use crate::handlers::note_handler::*;
use sqlx::PgPool;

pub fn note_routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_note_handler))
        .route("/:ticket_id", get(get_notes_by_ticket_handler))
}

use axum::{
    Router,
    routing::{post, get, patch, delete},
};
use crate::handlers::ticket_handler::*;
use sqlx::PgPool;

pub fn ticket_routes() -> Router<PgPool> {
    Router::new()
        .route("/", post(create_ticket_handler).get(get_all_tickets_handler))
        .route("/ticket/:id", get(get_ticket_handler))                      // ✅ Get 1 ticket
        .route("/ticket/:id/status", patch(update_ticket_status_handler))  // ✅ Update status
        .route("/ticket/:id/delete", delete(delete_ticket_handler))        // ✅ Delete ticket
}

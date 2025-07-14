use axum::{Router, routing::get};
use crate::handlers::report_handler::{
    ticket_overview_handler,
    tickets_by_agent_handler,
    ticket_status_count_handler,
};

use sqlx::PgPool; // ✅ Required for Router<PgPool>

pub fn report_routes() -> Router<PgPool> {
    Router::new()
        .route("/overview", get(ticket_overview_handler))
        .route("/by-agent", get(tickets_by_agent_handler))
        .route("/status-count", get(ticket_status_count_handler))
}

use axum_macros::debug_handler;

use axum::{
    extract::{Path, State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    models::ticket::{CreateTicketInput, UpdateTicketInput},
    services::ticket_service,
    utils::auth::AuthUser,
};

/// Create a new ticket (POST /tickets)
pub async fn create_ticket_handler(
    State(pool): State<PgPool>,
    user: AuthUser,
    Json(input): Json<CreateTicketInput>,
) -> impl IntoResponse {
    match ticket_service::create_ticket(&pool, input, Uuid::parse_str(&user.user_id).unwrap()).await {
        Ok(ticket) => (StatusCode::CREATED, Json(ticket)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

/// Get all tickets (GET /tickets)
pub async fn get_all_tickets_handler(
    State(pool): State<PgPool>,
    _user: AuthUser,
) -> impl IntoResponse {
    match ticket_service::get_all_tickets(&pool).await {
        Ok(tickets) => Json(tickets).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

/// Get ticket by ID (GET /tickets/ticket/:id)
pub async fn get_ticket_handler(
    State(pool): State<PgPool>,
    _user: AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> impl IntoResponse {
    match ticket_service::get_ticket_by_id(&pool, ticket_id).await {
        Ok(ticket) => Json(ticket).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Ticket not found").into_response(),
    }
}

/// Update ticket status (PATCH /tickets/ticket/:id/status)
#[debug_handler]
pub async fn update_ticket_status_handler(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<Uuid>,
    _user: AuthUser,
    Json(input): Json<UpdateTicketInput>, // ✅ Now it’s the last argument
) -> impl IntoResponse {
    match ticket_service::update_ticket_status(&pool, ticket_id, input).await {
        Ok(ticket) => (StatusCode::OK, Json(ticket)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Update failed").into_response(),
    }
}


/// Delete a ticket (DELETE /tickets/ticket/:id/delete)
pub async fn delete_ticket_handler(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<Uuid>,
    _user: AuthUser,
) -> impl IntoResponse {
    match ticket_service::delete_ticket(&pool, ticket_id).await {
        Ok(_) => (StatusCode::OK, "Ticket deleted").into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Delete failed").into_response(),
    }
}

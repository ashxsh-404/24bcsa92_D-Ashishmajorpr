use axum::{
    extract::{State, Path, Json},
    response::IntoResponse,
    http::StatusCode,
};
use axum_macros::debug_handler;
use uuid::Uuid;
use sqlx::PgPool;

use crate::{
    services::collaboration_service,
    models::message::CreateMessageInput,
    utils::auth::AuthUser,
};

/// POST /collab/:ticket_id — Add a message to a ticket
#[debug_handler]
pub async fn post_message_handler(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<Uuid>,
    user: AuthUser,
    Json(input): Json<CreateMessageInput>, // ✅ must be last
) -> impl IntoResponse {
    let sender_id = Uuid::parse_str(&user.user_id).unwrap();

    match collaboration_service::add_message_to_ticket(&pool, ticket_id, sender_id, input).await {
        Ok(msg) => (StatusCode::CREATED, Json(msg)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// GET /collab/:ticket_id — Get all messages for a ticket
#[debug_handler]
pub async fn get_messages_handler(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<Uuid>,
    _user: AuthUser,
) -> impl IntoResponse {
    match collaboration_service::get_messages_by_ticket(&pool, ticket_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

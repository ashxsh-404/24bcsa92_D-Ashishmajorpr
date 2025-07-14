use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use axum_macros::debug_handler; // ✅ Important fix

use crate::{
    models::note::{CreateNoteInput, Note},
    services::note_service,
    utils::auth::AuthUser,
};

/// POST /notes — Create a note for a ticket
#[debug_handler]
pub async fn create_note_handler(
    State(pool): State<PgPool>,
    user: AuthUser,
    Json(input): Json<CreateNoteInput>, // ✅ now at the end
) -> impl IntoResponse {
    let author_id = Uuid::parse_str(&user.user_id).unwrap();

    match note_service::create_note(&pool, input, author_id).await {
        Ok(note) => (StatusCode::CREATED, Json(note)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

/// GET /notes/:ticket_id — Get all notes for a ticket
#[debug_handler]
pub async fn get_notes_by_ticket_handler(
    State(pool): State<PgPool>,
    Path(ticket_id): Path<Uuid>,
    _user: AuthUser,
) -> impl IntoResponse {
    match note_service::get_notes_by_ticket_id(&pool, ticket_id).await {
        Ok(notes) => Json(notes).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
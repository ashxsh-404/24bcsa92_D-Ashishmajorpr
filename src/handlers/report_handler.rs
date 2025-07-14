use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::services::report_service;

pub async fn ticket_overview_handler(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match report_service::get_ticket_overview(&pool).await {
        Ok(data) => Json(data).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn tickets_by_agent_handler(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match report_service::get_tickets_by_agent(&pool).await {
        Ok(data) => Json(data).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn ticket_status_count_handler(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match report_service::get_ticket_status_count(&pool).await {
        Ok(data) => Json(data).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

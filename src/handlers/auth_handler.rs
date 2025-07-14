use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;
use sqlx::PgPool;

use crate::models::user::{RegisterInput, LoginInput};
use crate::services::auth_service::{register_user, login_user};

pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(input): Json<RegisterInput>,
) -> impl IntoResponse {
    match register_user(&pool, input).await {
        Ok(user) => (StatusCode::CREATED, Json(json!({
            "id": user.id,
            "name": user.name,
            "email": user.email,
            "role": user.role
        }))),
        Err(e) => {
            eprintln!("Signup error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        }
    }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(input): Json<LoginInput>,
) -> impl IntoResponse {
    match login_user(&pool, input).await {
        Ok(token) => (StatusCode::OK, Json(json!({ "token": token }))),
        Err(e) => {
            eprintln!("Login error: {:?}", e);
            (StatusCode::UNAUTHORIZED, Json(json!({ "error": e.to_string() })))
        }
    }
}

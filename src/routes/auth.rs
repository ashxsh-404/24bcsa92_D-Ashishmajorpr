use axum::{Router, routing::post};
use sqlx::PgPool;
use crate::handlers::auth_handler::{signup_handler, login_handler};

pub fn auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
}

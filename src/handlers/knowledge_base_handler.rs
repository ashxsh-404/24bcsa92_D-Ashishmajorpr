use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
};
use uuid::Uuid;
use sqlx::PgPool;
use axum_macros::debug_handler;

use crate::{
    models::knowledge_base::{CreateArticleInput, Article, KnowledgeBaseQueryParams},
    services::knowledge_base_service,
    utils::auth::AuthUser,
};

/// POST /kb — Create a new article
#[debug_handler]
pub async fn create_article_handler(
    State(pool): State<PgPool>,
    user: AuthUser,
    Json(input): Json<CreateArticleInput>,
) -> impl IntoResponse {
    let author_id = Uuid::parse_str(&user.user_id).unwrap();

    match knowledge_base_service::create_article(&pool, input, author_id).await {
        Ok(article) => (StatusCode::CREATED, Json(article)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

/// GET /kb — Filter/Search articles
pub async fn get_all_articles_handler(
    State(pool): State<PgPool>,
    Query(params): Query<KnowledgeBaseQueryParams>,
) -> impl IntoResponse {
    match knowledge_base_service::get_filtered_articles(&pool, params).await {
        Ok(articles) => Json(articles).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

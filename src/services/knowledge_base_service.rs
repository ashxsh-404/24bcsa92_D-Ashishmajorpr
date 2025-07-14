use crate::models::knowledge_base::{Article, CreateArticleInput, KnowledgeBaseQueryParams};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

/// Create new article
pub async fn create_article(
    pool: &PgPool,
    input: CreateArticleInput,
    author_id: Uuid,
) -> Result<Article, sqlx::Error> {
    let article = sqlx::query_as_unchecked!(
        Article,
        r#"
        INSERT INTO articles (id, title, content, tags, author_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, title, content, tags, author_id, created_at
        "#,
        Uuid::new_v4(),
        input.title,
        input.content,
        &input.tags,
        author_id,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(article)
}

/// Get filtered articles with optional search and tag filter
pub async fn get_filtered_articles(
    pool: &PgPool,
    params: KnowledgeBaseQueryParams,
) -> Result<Vec<Article>, sqlx::Error> {
    let mut query = String::from(
        "SELECT id, title, content, tags, author_id, created_at FROM articles WHERE 1=1",
    );

    if let Some(search) = &params.search {
        query.push_str(&format!(
            " AND (title ILIKE '%{}%' OR content ILIKE '%{}%')",
            search, search
        ));
    }

    if let Some(tag) = &params.tag {
        query.push_str(&format!(" AND '{}' = ANY(tags)", tag));
    }

    query.push_str(" ORDER BY created_at DESC");

    let articles = sqlx::query_as::<_, Article>(&query)
        .fetch_all(pool)
        .await?;

    Ok(articles)
}

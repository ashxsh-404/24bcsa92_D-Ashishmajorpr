use crate::models::note::{Note, CreateNoteInput};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_note(
    pool: &PgPool,
    input: CreateNoteInput,
    author_id: Uuid,
) -> Result<Note, sqlx::Error> {
    let note = sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (id, ticket_id, author_id, content)
        VALUES ($1, $2, $3, $4)
        RETURNING id, ticket_id, author_id, content, created_at
        "#,
        Uuid::new_v4(),
        input.ticket_id,
        author_id,
        input.content
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

pub async fn get_notes_by_ticket_id(
    pool: &PgPool,
    ticket_id: Uuid,
) -> Result<Vec<Note>, sqlx::Error> {
    let notes = sqlx::query_as!(
        Note,
        r#"
        SELECT id, ticket_id, author_id, content, created_at
        FROM notes
        WHERE ticket_id = $1
        ORDER BY created_at DESC
        "#,
        ticket_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

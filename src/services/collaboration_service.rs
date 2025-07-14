use crate::models::message::{Message, MessageWithSender, CreateMessageInput};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub async fn add_message_to_ticket(
    pool: &PgPool,
    ticket_id: Uuid,
    sender_id: Uuid,
    input: CreateMessageInput,
) -> Result<Message, sqlx::Error> {
    let message = sqlx::query_as_unchecked!(
        Message,
        r#"
        INSERT INTO messages (id, ticket_id, sender_id, content, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, ticket_id, sender_id, content, created_at
        "#,
        Uuid::new_v4(),
        ticket_id,
        sender_id,
        input.content,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(message)
}pub async fn get_messages_by_ticket(
    pool: &PgPool,
    ticket_id: Uuid,
) -> Result<Vec<MessageWithSender>, sqlx::Error> {
    let messages = sqlx::query_as_unchecked!(
        MessageWithSender,
        r#"
        SELECT 
            m.content, 
            u.name AS sender_name, 
            u.email AS sender_email, 
            m.created_at
        FROM messages m
        JOIN users u ON m.sender_id = u.id
        WHERE m.ticket_id = $1
        ORDER BY m.created_at ASC
        "#,
        ticket_id
    )
    .fetch_all(pool)
    .await?;

    Ok(messages)
}




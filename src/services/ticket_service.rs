use crate::models::ticket::{CreateTicketInput, Ticket, UpdateTicketInput};
use sqlx::{PgPool};
use uuid::Uuid;
use chrono::Utc;

pub async fn create_ticket(pool: &PgPool, input: CreateTicketInput, user_id: Uuid) -> Result<Ticket, sqlx::Error> {
    let ticket = sqlx::query_as!(
        Ticket,
        r#"
        INSERT INTO tickets (id, title, description, status, priority, created_by, created_at)
        VALUES ($1, $2, $3, 'open', $4, $5, $6)
        RETURNING id, title, description, status, priority, created_by, created_at
        "#,
        Uuid::new_v4(),
        input.title,
        input.description,
        input.priority,
        user_id,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(ticket)
}

pub async fn get_all_tickets(pool: &PgPool) -> Result<Vec<Ticket>, sqlx::Error> {
    let tickets = sqlx::query_as!(
        Ticket,
        r#"
        SELECT id, title, description, status, priority, created_by, created_at
        FROM tickets
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(tickets)
}

pub async fn get_ticket_by_id(pool: &PgPool, ticket_id: Uuid) -> Result<Ticket, sqlx::Error> {
    sqlx::query_as!(
        Ticket,
        r#"
        SELECT id, title, description, status, priority, created_by, created_at
        FROM tickets
        WHERE id = $1
        "#,
        ticket_id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_ticket_status(pool: &PgPool, ticket_id: Uuid, input: UpdateTicketInput) -> Result<Ticket, sqlx::Error> {
    sqlx::query_as!(
        Ticket,
        r#"
        UPDATE tickets
        SET status = $1
        WHERE id = $2
        RETURNING id, title, description, status, priority, created_by, created_at
        "#,
        input.status,
        ticket_id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_ticket(pool: &PgPool, ticket_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM tickets WHERE id = $1",
        ticket_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

use sqlx::PgPool;
use crate::models::report::{TicketOverview, AgentTicketCount, StatusCount};

pub async fn get_ticket_overview(pool: &PgPool) -> Result<TicketOverview, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) AS total,
            COUNT(*) FILTER (WHERE status = 'open') AS open,
            COUNT(*) FILTER (WHERE status = 'closed') AS closed,
            COUNT(*) FILTER (WHERE status = 'pending') AS pending
        FROM tickets
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(TicketOverview {
        total_tickets: row.total.unwrap_or(0),
        open_tickets: row.open.unwrap_or(0),
        closed_tickets: row.closed.unwrap_or(0),
        pending_tickets: row.pending.unwrap_or(0),
    })
}

pub async fn get_tickets_by_agent(pool: &PgPool) -> Result<Vec<AgentTicketCount>, sqlx::Error> {
    let results = sqlx::query!(
        r#"
        SELECT u.id AS agent_id, u.name AS agent_name, COUNT(t.id) AS ticket_count
        FROM users u
        LEFT JOIN tickets t ON u.id = t.assigned_to
        WHERE u.role = 'agent'
        GROUP BY u.id, u.name
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .map(|row| AgentTicketCount {
            agent_id: row.agent_id,
            agent_name: row.agent_name,
            ticket_count: row.ticket_count.unwrap_or(0),
        })
        .collect())
}

pub async fn get_ticket_status_count(pool: &PgPool) -> Result<Vec<StatusCount>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT status, COUNT(*) as count
        FROM tickets
        GROUP BY status
        ORDER BY count DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| StatusCount {
            status: row.status,
            count: row.count.unwrap_or(0),
        })
        .collect())
}

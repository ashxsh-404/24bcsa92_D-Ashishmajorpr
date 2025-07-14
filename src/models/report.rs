use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct TicketOverview {
    pub total_tickets: i64,
    pub open_tickets: i64,
    pub closed_tickets: i64,
    pub pending_tickets: i64,
}

#[derive(Debug, Serialize)]
pub struct AgentTicketCount {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub ticket_count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

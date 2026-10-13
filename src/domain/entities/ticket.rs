use uuid::Uuid;

use crate::domain::values::{TicketPriority, TicketStatus};

pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub created_at: chrono::NaiveDateTime,
}

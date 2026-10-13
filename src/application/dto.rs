use crate::domain::values::{TicketPriority, TicketStatus};

pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
}

#[derive(Debug, Default)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
    pub status: Option<TicketStatus>,
}

#[derive(Debug)]
pub struct Pagination {
    pub page: u64,
    pub limit: u64,
}

#[derive(Debug, Default)]
pub struct ListTicket {
    pub status: Option<TicketStatus>,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Default)]
pub struct TicketStats {
    pub open: u64,
    pub in_progress: u64,
    pub closed: u64,
}

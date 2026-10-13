use serde::{Deserialize, Serialize};

use crate::domain::values::{TicketPriority, TicketStatus};

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<TicketPriority>,
    pub status: Option<TicketStatus>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ListTicket {
    pub status: Option<TicketStatus>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl ListTicket {
    /// Rows to skip for the requested page. Pages are 1-based; without a `limit` nothing is skipped.
    pub fn offset(&self) -> u64 {
        match self.limit {
            Some(limit) => self
                .page
                .unwrap_or(1)
                .saturating_sub(1)
                .saturating_mul(limit),
            None => 0,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct TicketStats {
    pub open: u64,
    pub in_progress: u64,
    pub closed: u64,
}

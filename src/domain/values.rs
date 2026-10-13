#[derive(Debug, Clone, Copy)]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug, Clone, Copy)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}

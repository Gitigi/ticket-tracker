#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

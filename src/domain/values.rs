#[derive(Debug)]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[derive(Debug)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}

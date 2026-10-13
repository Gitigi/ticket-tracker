use std::sync::RwLock;

use crate::domain;

mod ticket;

pub struct MemoryRepository {
    tickets: RwLock<Vec<domain::Ticket>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        Self {
            tickets: RwLock::new(Vec::new()),
        }
    }
}

use std::sync::{Mutex, RwLock};

use crate::domain;

mod ticket;

pub struct MemoryPersistance {
    tickets: RwLock<Vec<domain::Ticket>>,
}

impl MemoryPersistance {
    pub fn new() -> Self {
        Self {
            tickets: RwLock::new(Vec::new()),
        }
    }
}

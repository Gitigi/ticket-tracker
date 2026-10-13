use std::sync::RwLock;

use sqlx::PgPool;

use crate::{application::AppError, domain};

mod memory_ticket;
mod ticket;

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

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

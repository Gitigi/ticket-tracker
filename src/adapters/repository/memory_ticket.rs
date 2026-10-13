use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    adapters::repository::MemoryRepository,
    application::{AppResult, TicketRepository, app_error::AppError, dto},
    domain,
};

#[async_trait]
impl TicketRepository for MemoryRepository {
    async fn create(&self, ticket: &dto::CreateTicket) -> AppResult<domain::Ticket> {
        let ticket = domain::Ticket {
            id: uuid::Uuid::now_v7(),
            title: ticket.title.clone(),
            description: ticket.description.clone(),
            status: domain::TicketStatus::Open,
            priority: ticket.priority,
            created_at: Utc::now().naive_utc(),
        };

        let mut lock = self.tickets.write().expect("mutex poisoned");

        lock.push(ticket.clone());

        Ok(ticket)
    }
    async fn update(&self, id: Uuid, update: dto::UpdateTicket) -> AppResult<()> {
        let mut lock = self.tickets.write().unwrap();
        let Some(ticket) = lock.iter_mut().find(|ticket| ticket.id == id) else {
            return Err(AppError::Database("ticket not found".into()));
        };
        if let Some(title) = update.title {
            ticket.title = title;
        }
        if let Some(description) = update.description {
            ticket.description = Some(description);
        }
        if let Some(status) = update.status {
            ticket.status = status;
        }
        if let Some(priority) = update.priority {
            ticket.priority = priority;
        }
        Ok(())
    }
    async fn get(&self, id: Uuid) -> AppResult<domain::Ticket> {
        let lock = self.tickets.read().unwrap();
        let Some(ticket) = lock.iter().find(|ticket| ticket.id == id) else {
            return Err(AppError::Database("ticket not found".into()));
        };
        Ok(ticket.clone())
    }
    async fn list(&self, query: &dto::ListTicket) -> AppResult<Vec<domain::Ticket>> {
        let lock = self.tickets.read().unwrap();
        let offset = usize::try_from(query.offset()).unwrap_or(usize::MAX);
        let limit = query.limit.map_or(usize::MAX, |limit| {
            usize::try_from(limit).unwrap_or(usize::MAX)
        });
        let tickets = lock
            .iter()
            .filter(|ticket| query.status.is_none_or(|status| ticket.status == status))
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();
        Ok(tickets)
    }
}

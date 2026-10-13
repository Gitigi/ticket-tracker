use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::{
        app_error::AppResult,
        dto::{self, CreateTicket, ListTicket, TicketStats, UpdateTicket},
    },
    domain::{self, TicketPriority, TicketStatus},
};

#[async_trait]
pub trait TicketRepository: Send + Sync {
    async fn create(&self, ticket: &CreateTicket) -> AppResult<domain::Ticket>;
    async fn update(&self, id: Uuid, ticket: UpdateTicket) -> AppResult<()>;
    async fn get(&self, id: Uuid) -> AppResult<domain::Ticket>;
    async fn list(&self, query: &ListTicket) -> AppResult<Vec<domain::Ticket>>;
}

#[derive(Clone)]
pub struct TicketUseCases {
    repository: Arc<dyn TicketRepository>,
}

impl TicketUseCases {
    pub fn new(repository: Arc<dyn TicketRepository>) -> Self {
        Self { repository }
    }

    pub async fn create(&self, ticket: CreateTicket) -> AppResult<domain::Ticket> {
        let ticket = self.repository.create(&ticket).await?;
        Ok(ticket)
    }

    pub async fn ticket(&self, id: Uuid) -> AppResult<domain::Ticket> {
        let ticket = self.repository.get(id).await?;
        Ok(ticket)
    }

    pub async fn list_tickets(&self, query: ListTicket) -> AppResult<Vec<domain::Ticket>> {
        self.repository.list(&query).await
    }

    pub async fn update(&self, id: Uuid, payload: UpdateTicket) -> AppResult<()> {
        self.repository.update(id, payload).await?;
        Ok(())
    }

    pub async fn update_status(&self, id: Uuid, status: TicketStatus) -> AppResult<()> {
        self.repository
            .update(
                id,
                UpdateTicket {
                    status: Some(status),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    pub async fn update_priority(&self, id: Uuid, priority: TicketPriority) -> AppResult<()> {
        self.repository
            .update(
                id,
                UpdateTicket {
                    priority: Some(priority),
                    ..Default::default()
                },
            )
            .await?;
        Ok(())
    }

    pub async fn stats(&self) -> AppResult<TicketStats> {
        let tickets = self.repository.list(&Default::default()).await?;
        let mut stat = dto::TicketStats::default();
        tickets.iter().for_each(|ticket| match ticket.status {
            TicketStatus::Open => stat.open += 1,
            TicketStatus::InProgress => stat.in_progress += 1,
            TicketStatus::Closed => stat.closed += 1,
        });
        Ok(stat)
    }
}

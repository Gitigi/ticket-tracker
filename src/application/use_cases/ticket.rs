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
pub trait TicketPersistence: Send + Sync {
    async fn create(&self, ticket: &CreateTicket) -> AppResult<domain::Ticket>;
    async fn update(&self, id: Uuid, ticket: UpdateTicket) -> AppResult<()>;
    async fn get(&self, id: Uuid) -> AppResult<domain::Ticket>;
    async fn list(&self, query: &ListTicket) -> AppResult<Vec<domain::Ticket>>;
}

#[derive(Clone)]
pub struct TicketUseCases {
    persistence: Arc<dyn TicketPersistence>,
}

impl TicketUseCases {
    pub fn new(persistence: Arc<dyn TicketPersistence>) -> Self {
        Self { persistence }
    }

    pub async fn create(&self, ticket: CreateTicket) -> AppResult<domain::Ticket> {
        let ticket = self.persistence.create(&ticket).await?;
        Ok(ticket)
    }

    pub async fn list_tickets(&self, query: ListTicket) -> AppResult<Vec<domain::Ticket>> {
        self.persistence.list(&query).await
    }

    pub async fn update_status(&self, id: Uuid, status: TicketStatus) -> AppResult<()> {
        self.persistence
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
        self.persistence
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
        let tickets = self.persistence.list(&Default::default()).await?;
        let mut stat = dto::TicketStats::default();
        tickets.iter().for_each(|ticket| match ticket.status {
            TicketStatus::Open => stat.open += 1,
            TicketStatus::InProgress => stat.in_progress += 1,
            TicketStatus::Closed => stat.closed += 1,
        });
        todo!()
    }
}

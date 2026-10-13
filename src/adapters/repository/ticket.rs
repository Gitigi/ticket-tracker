use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    adapters::repository::PostgresRepository,
    application::{AppResult, TicketRepository, app_error::AppError, dto},
    domain::{self, TicketPriority, TicketStatus},
};

struct TicketRow {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String,
    priority: String,
    created_at: NaiveDateTime,
}

#[async_trait]
impl TicketRepository for PostgresRepository {
    async fn create(&self, ticket: &dto::CreateTicket) -> AppResult<domain::Ticket> {
        let row = sqlx::query_as!(
            TicketRow,
            r#"
            INSERT INTO tickets (id, title, description, status, priority)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, status, priority, created_at
            "#,
            Uuid::now_v7(),
            ticket.title,
            ticket.description,
            status_to_db(TicketStatus::Open),
            priority_to_db(ticket.priority),
        )
        .fetch_one(&self.pool)
        .await?;

        row.try_into()
    }
    async fn update(&self, id: Uuid, update: dto::UpdateTicket) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            UPDATE tickets
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                priority = COALESCE($5, priority)
            WHERE id = $1
            "#,
            id,
            update.title,
            update.description,
            update.status.map(status_to_db),
            update.priority.map(priority_to_db),
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::Database("ticket not found".into()));
        }
        Ok(())
    }
    async fn get(&self, id: Uuid) -> AppResult<domain::Ticket> {
        let row = sqlx::query_as!(
            TicketRow,
            r#"
            SELECT id, title, description, status, priority, created_at
            FROM tickets
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Err(AppError::Database("ticket not found".into()));
        };
        row.try_into()
    }
    async fn list(&self, query: &dto::ListTicket) -> AppResult<Vec<domain::Ticket>> {
        let rows = sqlx::query_as!(
            TicketRow,
            r#"
            SELECT id, title, description, status, priority, created_at
            FROM tickets
            WHERE $1::text IS NULL OR status = $1
            ORDER BY created_at
            LIMIT $2 OFFSET $3
            "#,
            query.status.map(status_to_db),
            query.limit.map(to_i64),
            to_i64(query.offset()),
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(TryInto::try_into).collect()
    }
}

impl TryFrom<TicketRow> for domain::Ticket {
    type Error = AppError;

    fn try_from(row: TicketRow) -> AppResult<Self> {
        Ok(domain::Ticket {
            id: row.id,
            title: row.title,
            description: row.description,
            status: status_from_db(&row.status)?,
            priority: priority_from_db(&row.priority)?,
            created_at: row.created_at,
        })
    }
}

fn status_to_db(status: TicketStatus) -> &'static str {
    match status {
        TicketStatus::Open => "open",
        TicketStatus::InProgress => "in_progress",
        TicketStatus::Closed => "closed",
    }
}

fn status_from_db(status: &str) -> AppResult<TicketStatus> {
    match status {
        "open" => Ok(TicketStatus::Open),
        "in_progress" => Ok(TicketStatus::InProgress),
        "closed" => Ok(TicketStatus::Closed),
        other => Err(AppError::Database(format!(
            "unknown ticket status: {other}"
        ))),
    }
}

fn priority_to_db(priority: TicketPriority) -> &'static str {
    match priority {
        TicketPriority::Low => "low",
        TicketPriority::Medium => "medium",
        TicketPriority::High => "high",
    }
}

fn priority_from_db(priority: &str) -> AppResult<TicketPriority> {
    match priority {
        "low" => Ok(TicketPriority::Low),
        "medium" => Ok(TicketPriority::Medium),
        "high" => Ok(TicketPriority::High),
        other => Err(AppError::Database(format!(
            "unknown ticket priority: {other}"
        ))),
    }
}

fn to_i64(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

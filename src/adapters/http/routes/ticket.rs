use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
};
use uuid::Uuid;

use crate::{
    adapters::http::app_state::AppState,
    application::{AppResult, TicketUseCases, dto},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/", get(tickets))
        .route("/{id}", get(ticket))
        .route("/{id}", patch(update))
        .route("/stats", get(stats))
}

pub async fn create(
    State(user_use_cases): State<Arc<TicketUseCases>>,
    Json(payload): Json<dto::CreateTicket>,
) -> AppResult<impl IntoResponse> {
    let ticket = user_use_cases.create(payload).await?;
    Ok((StatusCode::CREATED, Json(ticket)))
}

pub async fn tickets(
    State(user_use_cases): State<Arc<TicketUseCases>>,
    Query(payload): Query<dto::ListTicket>,
) -> AppResult<impl IntoResponse> {
    let tickets = user_use_cases.list_tickets(payload).await?;
    Ok((StatusCode::OK, Json(tickets)))
}

pub async fn ticket(
    Path(id): Path<Uuid>,
    State(user_use_cases): State<Arc<TicketUseCases>>,
) -> AppResult<impl IntoResponse> {
    let ticket = user_use_cases.ticket(id).await?;
    Ok((StatusCode::OK, Json(ticket)))
}

pub async fn update(
    Path(id): Path<Uuid>,
    State(user_use_cases): State<Arc<TicketUseCases>>,
    Json(payload): Json<dto::UpdateTicket>,
) -> AppResult<impl IntoResponse> {
    let ticket = user_use_cases.update(id, payload).await?;
    Ok((StatusCode::OK, Json(ticket)))
}

pub async fn stats(
    State(user_use_cases): State<Arc<TicketUseCases>>,
) -> AppResult<impl IntoResponse> {
    let ticket = user_use_cases.stats().await?;
    Ok((StatusCode::OK, Json(ticket)))
}

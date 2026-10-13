use std::sync::Arc;

use crate::{adapters::http::app_state::AppState, application::TicketUseCases};

pub async fn init_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let postgres_repository = Arc::new(super::postgres_repository().await?);

    let user_use_cases = TicketUseCases::new(postgres_repository);

    Ok(AppState {
        ticket_use_cases: Arc::new(user_use_cases),
    })
}

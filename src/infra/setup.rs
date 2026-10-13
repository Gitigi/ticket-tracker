use std::sync::Arc;

use crate::{
    adapters::{http::app_state::AppState, repository::MemoryRepository},
    application::TicketUseCases,
};

pub async fn init_app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let repository = MemoryRepository::new();
    let user_use_cases = TicketUseCases::new(Arc::new(repository));

    Ok(AppState {
        ticket_use_cases: Arc::new(user_use_cases),
    })
}

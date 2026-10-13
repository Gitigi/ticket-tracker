use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::TicketUseCases;

#[derive(Clone)]
pub struct AppState {
    pub ticket_use_cases: Arc<TicketUseCases>,
}

impl FromRef<AppState> for Arc<TicketUseCases> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.ticket_use_cases.clone()
    }
}

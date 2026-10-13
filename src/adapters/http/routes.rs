pub mod ticket;

use axum::Router;

use crate::adapters::http::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/tickets", ticket::router())
}

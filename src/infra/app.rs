use axum::{Router, http};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;

use crate::adapters::{self, http::app_state::AppState};

pub fn create_app(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:3000"
                .parse::<http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([http::Method::POST, http::Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    Router::new()
        .nest("/api", adapters::http::routes::router())
        .with_state(app_state)
        .layer(cors)
}

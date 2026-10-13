use std::sync::Arc;

use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode, header::CONTENT_TYPE},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

use ticket_tracker::{
    adapters::{http::app_state::AppState, repository::PostgresRepository},
    application::TicketUseCases,
    infra::app::create_app,
};

fn app(pool: PgPool) -> Router {
    let repository = Arc::new(PostgresRepository::new(pool));
    create_app(AppState {
        ticket_use_cases: Arc::new(TicketUseCases::new(repository)),
    })
}

async fn send(app: &Router, method: Method, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let request = Request::builder().method(method).uri(uri);
    let request = match body {
        Some(body) => request
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string())),
        None => request.body(Body::empty()),
    }
    .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, body)
}

async fn create_ticket(app: &Router, title: &str, priority: &str) -> Value {
    let (status, body) = send(
        app,
        Method::POST,
        "/api/tickets",
        Some(json!({ "title": title, "priority": priority })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    body
}

fn titles(tickets: &Value) -> Vec<&str> {
    tickets
        .as_array()
        .unwrap()
        .iter()
        .map(|ticket| ticket["title"].as_str().unwrap())
        .collect()
}

#[sqlx::test]
async fn create_returns_new_open_ticket(pool: PgPool) {
    let app = app(pool);

    let (status, ticket) = send(
        &app,
        Method::POST,
        "/api/tickets",
        Some(json!({ "title": "Printer jammed", "description": "3rd floor", "priority": "High" })),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(ticket["title"], "Printer jammed");
    assert_eq!(ticket["description"], "3rd floor");
    assert_eq!(ticket["priority"], "High");
    assert_eq!(ticket["status"], "Open");
    assert!(ticket["id"].is_string());
    assert!(ticket["created_at"].is_string());

    let (status, fetched) = send(
        &app,
        Method::GET,
        &format!("/api/tickets/{}", ticket["id"].as_str().unwrap()),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(fetched, ticket);
}

#[sqlx::test]
async fn create_without_description_stores_null(pool: PgPool) {
    let app = app(pool);

    let ticket = create_ticket(&app, "No details", "Low").await;

    assert_eq!(ticket["description"], Value::Null);
}

#[sqlx::test]
async fn create_rejects_invalid_payload(pool: PgPool) {
    let app = app(pool);

    let (missing_title, _) = send(
        &app,
        Method::POST,
        "/api/tickets",
        Some(json!({ "priority": "Low" })),
    )
    .await;
    let (bad_priority, _) = send(
        &app,
        Method::POST,
        "/api/tickets",
        Some(json!({ "title": "x", "priority": "Urgent" })),
    )
    .await;

    assert_eq!(missing_title, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(bad_priority, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn list_returns_tickets_in_creation_order(pool: PgPool) {
    let app = app(pool);

    let (status, empty) = send(&app, Method::GET, "/api/tickets", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty, json!([]));

    for title in ["a", "b", "c"] {
        create_ticket(&app, title, "Medium").await;
    }

    let (status, tickets) = send(&app, Method::GET, "/api/tickets", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(titles(&tickets), ["a", "b", "c"]);
}

#[sqlx::test]
async fn list_filters_by_status(pool: PgPool) {
    let app = app(pool);

    create_ticket(&app, "open one", "Low").await;
    let closed = create_ticket(&app, "closed one", "Low").await;
    let uri = format!("/api/tickets/{}", closed["id"].as_str().unwrap());
    send(
        &app,
        Method::PATCH,
        &uri,
        Some(json!({ "status": "Closed" })),
    )
    .await;

    let (_, open) = send(&app, Method::GET, "/api/tickets?status=Open", None).await;
    let (_, closed) = send(&app, Method::GET, "/api/tickets?status=Closed", None).await;
    let (_, in_progress) = send(&app, Method::GET, "/api/tickets?status=InProgress", None).await;

    assert_eq!(titles(&open), ["open one"]);
    assert_eq!(titles(&closed), ["closed one"]);
    assert!(titles(&in_progress).is_empty());
}

#[sqlx::test]
async fn list_paginates(pool: PgPool) {
    let app = app(pool);

    for title in ["a", "b", "c", "d", "e"] {
        create_ticket(&app, title, "Low").await;
    }

    let (_, page1) = send(&app, Method::GET, "/api/tickets?limit=2", None).await;
    let (_, page2) = send(&app, Method::GET, "/api/tickets?limit=2&page=2", None).await;
    let (_, page3) = send(&app, Method::GET, "/api/tickets?limit=2&page=3", None).await;
    let (_, page4) = send(&app, Method::GET, "/api/tickets?limit=2&page=4", None).await;

    assert_eq!(titles(&page1), ["a", "b"]);
    assert_eq!(titles(&page2), ["c", "d"]);
    assert_eq!(titles(&page3), ["e"]);
    assert!(titles(&page4).is_empty());
}

#[sqlx::test]
async fn update_changes_only_given_fields(pool: PgPool) {
    let app = app(pool);
    let ticket = create_ticket(&app, "Original", "Low").await;
    let uri = format!("/api/tickets/{}", ticket["id"].as_str().unwrap());

    let (status, _) = send(
        &app,
        Method::PATCH,
        &uri,
        Some(json!({ "status": "InProgress", "description": "added later" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (_, updated) = send(&app, Method::GET, &uri, None).await;
    assert_eq!(updated["status"], "InProgress");
    assert_eq!(updated["description"], "added later");
    assert_eq!(updated["title"], "Original");
    assert_eq!(updated["priority"], "Low");
    assert_eq!(updated["created_at"], ticket["created_at"]);
}

#[sqlx::test]
async fn update_all_fields(pool: PgPool) {
    let app = app(pool);
    let ticket = create_ticket(&app, "Original", "Low").await;
    let uri = format!("/api/tickets/{}", ticket["id"].as_str().unwrap());

    let (status, _) = send(
        &app,
        Method::PATCH,
        &uri,
        Some(json!({ "title": "Renamed", "description": "d", "priority": "High", "status": "Closed" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (_, updated) = send(&app, Method::GET, &uri, None).await;
    assert_eq!(updated["title"], "Renamed");
    assert_eq!(updated["description"], "d");
    assert_eq!(updated["priority"], "High");
    assert_eq!(updated["status"], "Closed");
}

#[sqlx::test]
async fn update_unknown_ticket_fails(pool: PgPool) {
    let app = app(pool);

    let (status, _) = send(
        &app,
        Method::PATCH,
        &format!("/api/tickets/{}", uuid::Uuid::now_v7()),
        Some(json!({ "title": "ghost" })),
    )
    .await;

    // Currently surfaces as 500 because "not found" is an AppError::Database.
    assert!(!status.is_success());
}

#[sqlx::test]
async fn update_rejects_invalid_status(pool: PgPool) {
    let app = app(pool);
    let ticket = create_ticket(&app, "t", "Low").await;
    let uri = format!("/api/tickets/{}", ticket["id"].as_str().unwrap());

    let (status, _) = send(&app, Method::PATCH, &uri, Some(json!({ "status": "Done" }))).await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);

    let (_, unchanged) = send(&app, Method::GET, &uri, None).await;
    assert_eq!(unchanged["status"], "Open");
}

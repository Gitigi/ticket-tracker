use ticket_tracker::infra::{app::create_app, setup::init_app_state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let app_state = init_app_state().await?;

    let app = create_app(app_state);

    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".into());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

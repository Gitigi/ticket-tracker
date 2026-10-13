use ticket_tracker::infra::{app::create_app, setup::init_app_state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = init_app_state().await?;

    let app = create_app(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Backend listening at {}", &listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

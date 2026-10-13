use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to database!");

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

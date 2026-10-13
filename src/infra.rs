use crate::adapters::repository::PostgresRepository;

pub mod app;
pub mod db;
pub mod setup;

pub async fn postgres_repository() -> Result<PostgresRepository, Box<dyn std::error::Error>> {
    let pool = db::init_db().await?;
    let persistence = PostgresRepository::new(pool);
    Ok(persistence)
}

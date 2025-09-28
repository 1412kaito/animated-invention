use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub mod config;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://lino:lino@localhost:5432/simple_api".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
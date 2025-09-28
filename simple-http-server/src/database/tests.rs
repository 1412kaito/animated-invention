#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_database_connection() {
        dotenv::dotenv().ok();

        let pool = create_pool().await;

        match pool {
            Ok(pool) => {
                let result = sqlx::query("SELECT 1")
                    .fetch_one(&pool)
                    .await;

                match result {
                    Ok(_) => println!("✅ Database connection successful"),
                    Err(e) => panic!("❌ Database query failed: {}", e),
                }
            }
            Err(e) => panic!("❌ Failed to create database pool: {}", e),
        }
    }

    #[tokio::test]
    async fn test_database_config_from_env() {
        dotenv::dotenv().ok();

        let config = config::DatabaseConfig::from_env();

        assert!(!config.url.is_empty());
        assert!(config.max_connections > 0);

        println!("✅ Database config loaded: {}", config.url);
    }
}
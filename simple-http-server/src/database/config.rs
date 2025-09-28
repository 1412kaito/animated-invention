use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://user:pasword@localhost:5432/database_name".to_string()),
            max_connections: 5,
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://lino:lino@localhost:5432/simple_api".to_string()),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .map(|s| s.parse().unwrap_or(5))
                .unwrap_or(5),
        }
    }
}
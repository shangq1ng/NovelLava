use crate::auth::models::AuthConfig;
use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub db: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub db: PgPool,
}

impl DatabaseConfig {
    pub async fn new() -> Result<Self> {
        let db_url =
            std::env::var("DATABASE_URL").context("Missing environment variable 'DATABASE_URL'")?;
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await?;

        Ok(DatabaseConfig { db: pool })
    }
}

use crate::models::cache::models::Cache;
use anyhow::{Context, Result};
use meilisearch_sdk::client::Client;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub meilisearch: MeilisearchConfig,
    pub cache: Cache,
}

#[derive(Clone, Debug)]
pub struct MeilisearchConfig {
    pub meilisearch: Client,
}

impl MeilisearchConfig {
    pub async fn new() -> Result<Self> {
        let meilisearch_url = env::var("MEILISEARCH_URL").context("missing MEILISEARCH_URL")?;
        let meilisearch_master_key =
            env::var("MEILISEARCH_MASTER_KEY").context("missing MEILISEARCH_MASTER_KEY")?;

        let m_client = Client::new(&meilisearch_url, Some(meilisearch_master_key))?;

        Ok(Self {
            meilisearch: m_client,
        })
    }
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

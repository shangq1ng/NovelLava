use anyhow::Context;
use deadpool_redis::{Config, Connection, Pool, Runtime};
use std::env;

#[derive(Debug, Clone)]
pub struct Cache {
    pub pool: Pool,
}

impl Cache {
    pub async fn new() -> anyhow::Result<Self> {
        let url = env::var("CACHE_REDIS_URL").context("CACHE_REDIS_URL env var not set")?;
        let config = Config::from_url(&url);
        let pool = config.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> anyhow::Result<Connection> {
        Ok(self.pool.get().await?)
    }
}

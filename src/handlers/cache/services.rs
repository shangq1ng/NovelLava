use crate::models::config::models::AppConfig;
use crate::models::errors::models::CacheError;
use crate::models::meilisearch::models::MeilisearchResult;
use deadpool_redis::redis::AsyncTypedCommands;
use meilisearch_sdk::search::SearchResults;

pub async fn insert_paginated_results(
    config: &AppConfig,
    key: &str,
    res: &SearchResults<MeilisearchResult>,
) -> Result<bool, CacheError> {
    let mut conn = config.cache.get_connection().await?;
    let convert = serde_json::to_string(&res)?;
    if let Err(e) = conn.set_ex(key, convert, 600).await {
        Err(CacheError::RedisError(e))
    } else {
        Ok(true)
    }
}

pub async fn get_paginated_results(config: &AppConfig, key: &str) -> Result<String, CacheError> {
    let mut conn = config.cache.get_connection().await?;
    if let Some(res) = conn.get(key).await? {
        Ok(res)
    } else {
        Err(CacheError::Failed)
    }
}

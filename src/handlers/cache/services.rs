use crate::models::auth::models::User;
use crate::models::config::models::AppConfig;
use crate::models::errors::models::CacheError;
use deadpool_redis::redis::AsyncTypedCommands;
use tracing::warn;

pub async fn insert_user(
    config: &AppConfig,
    user: &User,
    key: &str,
) -> Result<Option<bool>, CacheError> {
    let mut conn = config.cache.get_connection().await?;
    let usr = serde_json::to_string(&user)?;
    if let Err(e) = conn.set(key, usr).await {
        warn!("Error while inserting user: {:?}", e);
    };
    Ok(Some(true))
}

pub async fn get_user(config: &AppConfig, key: &str) -> Result<Option<User>, CacheError> {
    let mut conn = config.cache.get_connection().await?;
    let usr = conn.get(key).await?;
    match usr {
        Some(user) => {
            let usr = serde_json::from_str(&user)?;
            Ok(usr)
        }
        None => Err(CacheError::Failed),
    }
}

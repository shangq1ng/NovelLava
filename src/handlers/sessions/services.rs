use crate::models::errors::models::AuthError;
use anyhow::Context;
use tower_sessions::cookie::SameSite;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tower_sessions_redis_store::fred::prelude::{ClientLike, Config, Pool};

pub async fn start_redis() -> Result<SessionManagerLayer<RedisStore<Pool>>, AuthError> {
    let url = std::env::var("REDIS_URL").context("REDIS_URL must be set")?;
    let config = Config::from_url(&url)?;
    let pool = Pool::new(config, None, None, None, 10)?;
    pool.connect();
    pool.wait_for_connect().await?;

    let session_manager = RedisStore::new(pool);

    let session_manager_layer = SessionManagerLayer::new(session_manager)
        .with_expiry(Expiry::OnInactivity(Duration::hours(5)))
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_http_only(true);

    Ok(session_manager_layer)
}

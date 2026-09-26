use crate::models::errors::models::AuthError;
use anyhow::Context;
use std::env;
use tower_sessions::cookie::time::Duration;
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions::service::PrivateCookie;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::RedisStore;
use tower_sessions_redis_store::fred::prelude::{ClientLike, Config, Pool};

pub async fn start_session_redis()
-> Result<SessionManagerLayer<RedisStore<Pool>, PrivateCookie>, AuthError> {
    let url = std::env::var("SESSION_REDIS_URL").context("SESSION_REDIS_URL must be set")?;
    let config = Config::from_url(&url)?;
    let pool = Pool::new(config, None, None, None, 10)?;
    pool.connect();
    pool.wait_for_connect().await?;

    // signed keys for .with_private() to ensure ID values are not visible in dev tool
    let key = env::var("SESSION_REDIS_SIGNED_KEY").context("SESSION_SIGNED_KEY must be set")?;
    let p_key = Key::try_from(key.as_bytes()).context("CACHE_SIGNED_KEY must be valid")?;
    // signed keys for .with_signed() to ensure cookies are tamper-evident.
    let key =
        env::var("SESSION_REDIS_PRIVATE_KEY").context("SESSION_REDIS_PRIVATE_KEY must be set")?;
    let s_key = Key::try_from(key.as_bytes()).context("CACHE_SIGNED_KEY must be valid")?;

    let session_manager = RedisStore::new(pool);

    let session_manager_layer = SessionManagerLayer::new(session_manager)
        .with_expiry(Expiry::OnInactivity(Duration::hours(5)))
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_signed(s_key)
        .with_private(p_key);

    Ok(session_manager_layer)
}

use crate::handlers::auth::service::{me, sign_in, sign_up, signin_page, signup_page};
use crate::handlers::helpers::services::{check_if_logged_in, home};
use crate::handlers::meilisearch::services::search;
use crate::handlers::sessions::services::start_session_redis;
use crate::models::cache::models::Cache;
use crate::models::config::models::{AppConfig, DatabaseConfig, MeilisearchConfig};
use crate::models::helpers::models::SmartExtractor;
use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use std::sync::Arc;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

pub mod handlers;
pub mod models;
pub async fn app() -> anyhow::Result<()> {
    match dotenv() {
        Ok(_) => tracing::info!("Successfully loaded .env"),
        Err(e) => panic!("Error loading .env, {e:?}"),
    }
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(fmt::layer())
        .init();

    let logged_in_ratelimit_cfg = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartExtractor)
            .per_second(15)
            .burst_size(25)
            .finish()
            .ok_or(anyhow::anyhow!("Failed to build rate limiter"))?,
    );
    let anon_ratelimit_cfg = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(10)
            .finish()
            .ok_or(anyhow::anyhow!("Failed to build rate limiter"))?,
    );

    let cache = Cache::new().await?;
    let session_redis = start_session_redis().await?;
    let db = DatabaseConfig::new().await?;
    let meilisearch = MeilisearchConfig::new().await?;
    let state = AppConfig {
        database: db,
        meilisearch,
        cache,
    };

    let router: Router = Router::new()
        .route("/", get(home))
        .route("/signup", get(signup_page))
        .route("/signin", get(signin_page))
        .route("/users/me", get(me))
        .route("/auth/signup", post(sign_up))
        .route("/auth/login", post(sign_in))
        .route("/api/search", get(search))
        .layer(GovernorLayer::new(logged_in_ratelimit_cfg))
        .layer(GovernorLayer::new(anon_ratelimit_cfg))
        .layer(axum::middleware::from_fn(check_if_logged_in))
        .layer(session_redis)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

use crate::handlers::auth::service::{me, sign_in, sign_up};
use crate::handlers::db::services::{
    find_users_by_username, update_avatar, update_bio, update_display_name,
};
use crate::handlers::helpers::services::check_if_logged_in;
use crate::handlers::meilisearch::services::search;
use crate::handlers::sessions::services::start_session_redis;
use crate::models::cache::models::Cache;
use crate::models::config::models::{AppConfig, DatabaseConfig, MeilisearchConfig};
use crate::models::helpers::models::SmartExtractor;
use crate::models::s3::models::S3BucketOperator;
use axum::Router;
use axum::http::header::{ACCEPT, AUTHORIZATION};
use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use dotenv::dotenv;
use reqwest::header::CONTENT_TYPE;
use std::sync::Arc;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use crate::handlers::db::review::services::{create_review, edit_review, get_all_child_reviews, get_all_reviews, reply_review};

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
    let auth_governer = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(4)
            .burst_size(2)
            .finish()
            .expect("failed to create governor config"),
    );

    let logged_in_ratelimit_cfg = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartExtractor)
            .per_millisecond(1110)
            .burst_size(25)
            .finish()
            .ok_or(anyhow::anyhow!("Failed to build rate limiter"))?,
    );
    let anon_ratelimit_cfg = Arc::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartExtractor)
            .per_millisecond(333)
            .burst_size(10)
            .finish()
            .ok_or(anyhow::anyhow!("Failed to build rate limiter"))?,
    );
    let auth_limiter = logged_in_ratelimit_cfg.limiter().clone();
    let anon_limiter = anon_ratelimit_cfg.limiter().clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            auth_limiter.retain_recent();
            anon_limiter.retain_recent();
        }
    });
    let cors_conf = CorsLayer::new()
        .allow_origin("http://127.0.0.1:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);
    let cache = Cache::new().await?;
    let session_redis = start_session_redis().await?;
    let db = DatabaseConfig::new().await?;
    let op = S3BucketOperator::new().await?;
    let meilisearch = MeilisearchConfig::new().await?;
    let state = AppConfig {
        database: db,
        meilisearch,
        cache,
        s3: op,
    };
    let api: Router = Router::new()
        .route("/search/{username}", get(find_users_by_username))
        .route("/search", get(search))
        .layer(GovernorLayer::new(logged_in_ratelimit_cfg))
        .layer(GovernorLayer::new(anon_ratelimit_cfg))
        .layer(axum::middleware::from_fn(check_if_logged_in))
        .layer(cors_conf)
        .layer(session_redis.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(state.clone());

    let auth: Router = Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(sign_in))
        .route("/review", post(create_review))
        .route("/review/reply", post(reply_review))
        .route("/review/content", post(edit_review))
        .route("/reviews", get(get_all_reviews))
        .route("/reviews/child", get(get_all_child_reviews))
        .route("/users/me", get(me))
        .route("/users/me/display-name", post(update_display_name))
        .route("/users/me/bio", post(update_bio))
        .route("/users/me/avatar", post(update_avatar))
        .layer(GovernorLayer::new(auth_governer))
        .layer(session_redis.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(DefaultOnResponse::new().level(Level::DEBUG))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
        )
        .with_state(state);

    let app: Router = Router::new().nest("/api", api).nest("/auth", auth);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

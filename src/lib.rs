use crate::handlers::auth::service::{home, me, sign_in, sign_up, signin_page, signup_page};
use crate::handlers::meilisearch::services::search;
use crate::handlers::sessions::services::start_redis;
use crate::middleware::models::RateLimit;
use crate::middleware::rate_limiter::rate_limiter;
use crate::models::config::models::{AppConfig, DatabaseConfig, MeilisearchConfig};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use dotenv::dotenv;
use tower_http::trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

pub mod handlers;
pub mod middleware;
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

    let rate_limit = RateLimit::new(10, 20).await;
    let redis = start_redis().await?;
    let db = DatabaseConfig::new().await?;
    let meilisearch = MeilisearchConfig::new().await?;
    let state = AppConfig {
        database: db,
        meilisearch,
    };

    let router: Router = Router::new()
        .route("/", get(home))
        .route("/signup", get(signup_page))
        .route("/signin", get(signin_page))
        .route("/users/me", get(me))
        .route("/auth/signup", post(sign_up))
        .route("/auth/login", post(sign_in))
        .route("/api/search", get(search))
        .route_layer(from_fn_with_state(rate_limit, rate_limiter))
        .layer(redis)
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

use crate::handlers::meilisearch::services::search;
use crate::middleware::models::RateLimit;
use crate::middleware::rate_limiter::rate_limiter;
use crate::models::config::models::{AppConfig, DatabaseConfig, MeilisearchConfig};
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use dotenv::dotenv;

pub mod handlers;
pub mod middleware;
pub mod models;
pub async fn app() -> anyhow::Result<()> {
    match dotenv() {
        Ok(_) => tracing::info!("Successfully loaded .env"),
        Err(e) => panic!("Error loading .env, {e:?}"),
    }
    let rate_limit = RateLimit::new(10, 10).await;

    let db = DatabaseConfig::new().await?;
    let meilisearch = MeilisearchConfig::new().await?;
    let state = AppConfig {
        database: db,
        meilisearch,
    };
    let router: Router = Router::new()
        .route("/api/search", get(search))
        .route_layer(from_fn_with_state(rate_limit, rate_limiter))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

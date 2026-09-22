use crate::models::config::models::{AppConfig, DatabaseConfig, MeilisearchConfig};
use axum::Router;
use axum::routing::get;
use crate::handlers::meilisearch::services::search;

pub mod handlers;
pub mod models;

pub async fn app() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let db = DatabaseConfig::new().await?;
    let meilisearch = MeilisearchConfig::new().await?;
    let state = AppConfig {
        database: db,
        meilisearch,
    };
    let router: Router = Router::new()
        .route("/api/search", get(search))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

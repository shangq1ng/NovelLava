use novellava::models::config::models::DatabaseConfig;
use novellava::models::db::models::NovelEntity;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};
use std::env;

#[derive(Debug, Serialize)]
pub struct Novel {
    pub id: Uuid,
    pub author: String,
    pub title: String,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub storage_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<NovelEntity> for Novel {
    fn from(entity: NovelEntity) -> Self {
        Self {
            id: entity.id,
            author: entity.author,
            title: entity.title,
            cover_path: entity.cover_path,
            description: entity.description,
            tags: entity.tags,
            storage_path: entity.storage_path,
            created_at: entity.created_at,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let meilisearch_url = env::var("MEILISEARCH_URL")?;
    let meilisearch_master_key = env::var("MEILISEARCH_MASTER_KEY")?;
    let db = DatabaseConfig::new().await?;
    let query = sqlx::query_as!(NovelEntity, "SELECT * FROM novels")
        .fetch_all(&db.db)
        .await?;

    let novel: Vec<Novel> = query.into_iter().map(|entity| entity.into()).collect();
    let client = meilisearch_sdk::client::Client::new(
        &meilisearch_url,
        Some(meilisearch_master_key.clone()),
    )?;

    let indexxer = client
        .index("novels")
        .add_or_replace(&[novel], Some("id"))
        .await?;

    indexxer.wait_for_completion(&client, None, None).await?;
    Ok(())
}

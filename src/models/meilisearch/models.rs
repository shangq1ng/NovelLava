use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeilisearchResult {
    pub slug: String,
    pub series_type_slug: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub author_slugs: Vec<String>,
    pub genre_slugs: Vec<String>,
    pub tags_slugs: Option<Vec<String>>,
    pub release_date: Option<DateTime<Utc>>,
    pub title: String,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub rating: Option<f64>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub per_page: Option<usize>,
    pub page: Option<usize>,
    pub filter_by_author: Option<String>,
    pub filter_by_tags: Option<String>,
    pub filter_by_genres: Option<String>,
    pub filter_by_series_type: Option<String>,
}


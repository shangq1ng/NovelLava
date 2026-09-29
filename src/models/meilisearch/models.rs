use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeilisearchResult {
    pub slug: Option<String>,
    pub series_type_slug: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
    pub author_slugs: Option<Vec<String>>,
    pub genre_slugs: Option<Vec<String>>,
    pub tags_slugs: Option<Vec<String>>,
    pub release_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub rating: Option<f64>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "hitsPerPage")]
    pub per_page: Option<usize>,
    pub page: Option<usize>,
    pub author: Option<String>,
    pub tags: Option<String>,
    pub genres: Option<String>,
    pub series: Option<String>,
}

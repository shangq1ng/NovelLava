use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MeilisearchResult {
    pub author: String,
    pub title: String,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub per_page: Option<usize>,
    pub page: Option<usize>,
    pub filter_by_author: Option<String>,
    pub filter_by_tags: Option<String>,
}

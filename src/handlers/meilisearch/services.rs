use crate::models::config::models::AppConfig;
use crate::models::errors::models::SearchError;
use crate::models::meilisearch::models::{MeilisearchResult, SearchQuery};
use axum::extract::Json;
use axum::extract::{Query, State};
use meilisearch_sdk::search::SearchResults;

pub async fn search(
    State(config): State<AppConfig>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResults<MeilisearchResult>>, SearchError> {
    let client = config.meilisearch.meilisearch.index("series");
    let mut search = client.search();
    let SearchQuery { q, per_page, page, filter_by_author, filter_by_tags, filter_by_genres, filter_by_series_type } = query;
    search.with_page(page.unwrap_or(1)).with_hits_per_page(per_page.unwrap_or(20));

    if let Some(v) = &q {
        search.with_query(v.as_str());
    }

    if let Some(v) = &filter_by_tags {
        search.with_filter(v.as_str());
    }

    if let Some(v) = &filter_by_genres {
        search.with_filter(v.as_str());
    }

    if let Some(v) = &filter_by_author {
        search.with_filter(v.as_str());
    }

    if let Some(v) = &filter_by_series_type {
        search.with_filter(v.as_str());
    }

    let result = search.execute().await?;

    Ok(Json(result))
}

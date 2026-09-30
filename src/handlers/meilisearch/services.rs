use crate::handlers::cache::services::{get_paginated_results, insert_paginated_results};
use crate::models::config::models::AppConfig;
use crate::models::errors::models::SearchError;
use crate::models::meilisearch::models::{MeilisearchResult, SearchQuery};
use axum::extract::Json;
use axum::extract::{Query, State};
use meilisearch_sdk::search::SearchResults;
use tracing::warn;

pub async fn search(
    State(config): State<AppConfig>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResults<MeilisearchResult>>, SearchError> {
    let client = config.meilisearch.meilisearch.index("series");
    let mut search = client.search();
    let SearchQuery {
        q,
        per_page,
        page,
        author,
        tags,
        genres,
        series,
        slug,
    } = query;

    if let Some(v) = &q {
        search.with_query(v);
    }

    if let Some(v) = &slug
        && is_valid(v)
    {
        let filter = format!("slug = {}", v).to_string();
        let mut search_once = client.search();
        let author = search_once.with_filter(filter.as_str()).execute().await?;
        return Ok(author.into());
    }

    if let Some(v) = &tags
        && is_valid(v)
    {
        let filter = format!("tag_slugs = {}", v).to_string();
        let mut search_once = client.search();
        let author = search_once.with_filter(filter.as_str()).execute().await?;
        return Ok(author.into());
    }

    if let Some(v) = &genres
        && is_valid(v)
    {
        let filter = format!("genre_slugs = {}", v).to_string();
        let mut search_once = client.search();
        let author = search_once.with_filter(filter.as_str()).execute().await?;
        return Ok(author.into());
    }

    if let Some(v) = &author
        && is_valid(v)
    {
        let filter = format!("author_slugs = {}", v).to_string();
        let mut search_once = client.search();
        let author = search_once.with_filter(filter.as_str()).execute().await?;
        return Ok(author.into());
    }

    if let Some(v) = &series
        && is_valid(v)
    {
        let filter = format!("series_type_slug = {}", v).to_string();
        let mut search_once = client.search();
        let author = search_once.with_filter(filter.as_str()).execute().await?;
        return Ok(author.into());
    }

    let cache_key = format!(
        "search:v1:browse:{}:{}",
        page.unwrap_or_default(), per_page.unwrap_or_default(),
    );
    if let Ok(v) = get_paginated_results(&config, cache_key.as_str()).await {
        let val = serde_json::from_str(v.as_str())?;
        return Ok(Json(val));
    } else {
        search
            .with_page(page.unwrap_or(1))
            .with_hits_per_page(per_page.unwrap_or(20));
    }

    let result = search.execute().await?;

    if check_if_query_for_pagination(&page, &per_page) == true {
        insert_paginated_results(&config, cache_key.as_str(), &result)
            .await
            .map_err(|e| warn!("Cache Error: {e:?}"))
            .unwrap_or_default();
        Ok(Json(result))
    } else {
        Ok(Json(result))
    }
}

pub fn is_valid(v: &str) -> bool {
    !v.is_empty()
        && v.len() <= 128
        && v.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub fn check_if_query_for_pagination(page: &Option<usize>, per_page: &Option<usize>) -> bool {
    if page.is_some() || per_page.is_some() {
        true
    } else {
        false
    }
}

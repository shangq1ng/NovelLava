use crate::models::config::models::AppConfig;
use crate::models::errors::models::SearchError;
use crate::models::meilisearch::models::{MeilisearchResult, SearchQuery};
use axum::extract::Json;
use axum::extract::{Query, State};
use meilisearch_sdk::search::SearchResults;
#[axum::debug_handler]
pub async fn search(
    State(config): State<AppConfig>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResults<MeilisearchResult>>, SearchError> {
    let client = config.meilisearch.meilisearch.index("novels");

    client
        .set_filterable_attributes(&["tags", "author"])
        .await?;

    let per_page = query.per_page.unwrap_or(5);
    let page = query.page.unwrap_or(1);

    // q
    if let Some(v) = query.q {
        client.search().with_query(v.as_str());
    }

    // per page | page
    client.search().with_page(page).with_hits_per_page(per_page);

    // filtered
    if let Some(v) = query.filter_by_tags {
        client.search().with_filter(v.as_str());
    }

    if let Some(v) = query.filter_by_author {
        client.search().with_filter(v.as_str());
    }

    let result = client.search().execute().await?;

    Ok(Json(result))
}

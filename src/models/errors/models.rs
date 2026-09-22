use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::SqlxError(e) => {
                error!("SQLx error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Internal(e) => {
                error!("Internal Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status.into_response()
    }
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("{0}")]
    Internal(#[source] anyhow::Error),
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    MeiliError(#[from] meilisearch_sdk::errors::Error),
    #[error(transparent)]
    SearchError(#[from] meilisearch_sdk::errors::MeilisearchError),
    #[error(transparent)]
    CommunicationError(#[from] meilisearch_sdk::errors::MeilisearchCommunicationError),
}

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::MeiliError(e) => {
                error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal(e) => {
                error!("Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::CommunicationError(e) => {
                error!("Meilisearch Communication Error {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::SearchError(e) => {
                error!("Search Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status.into_response()
    }
}

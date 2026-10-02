use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::multipart::MultipartError;
use deadpool_redis::redis::RedisError;
use thiserror::Error;
use tracing::error;

//TODO: Fix Status-Code semantic, refactor so not every error is 404/500/etc.

#[derive(Error, Debug)]
pub enum S3ObjectError {
    #[error(transparent)]
    S3Error(#[from] opendal::Error),
    #[error("Failed Operation")]
    Failed,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl IntoResponse for S3ObjectError {
    fn into_response(self) -> Response {
        let status = match self {
            S3ObjectError::SerdeJson(e) => {
                error!("Serde Json: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::S3Error(e) => {
                error!("S3 Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Failed => StatusCode::UNPROCESSABLE_ENTITY, // change to an appropriate status code at a later date.
            Self::Other(e) => {
                error!("Unexpected error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status.into_response()
    }
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error(transparent)]
    SerdeConversionError(#[from] serde_json::Error),
    #[error(transparent)]
    RedisError(#[from] RedisError),
    #[error("Failed Operation")]
    Failed,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for CacheError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::SerdeConversionError(e) => {
                tracing::error!("serde conversion error: {}", e);
                StatusCode::BAD_REQUEST
            }
            Self::RedisError(e) => {
                error!("Redis Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Failed => StatusCode::UNPROCESSABLE_ENTITY, // change to an appropriate status code at a later date.
            Self::Other(e) => {
                error!("Unexpected error: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status.into_response()
    }
}

#[derive(Error, Debug)]
pub enum MiddlewareError {
    #[error("{0}")]
    TooManyRequests(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    TowerSessionError(#[from] tower_sessions::session::Error),
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
}

impl IntoResponse for MiddlewareError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::TooManyRequests(e) => {
                error!("{e:?}``");
                StatusCode::TOO_MANY_REQUESTS
            }
            Self::Other(e) => {
                error!("Other Error{e:?}``");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::UuidError(e) => {
                error!("UUID Error {e:?}``");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NotFound => {
                dbg!(self);
                StatusCode::NOT_FOUND
            }
            Self::TowerSessionError(e) => {
                error!("Tower Session Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to post comment/review")]
    FailedToCreateReview,
    #[error(transparent)]
    MultipartError(#[from] MultipartError),
    #[error(transparent)]
    TowerSessionError(#[from] tower_sessions::session::Error),
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
            Self::FailedToCreateReview => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            Self::MultipartError(e) => {
                error!("Multipart Error: {e:?}");
                StatusCode::BAD_REQUEST
            }
            Self::TowerSessionError(e) => {
                error!("Tower Session Error {e:?}``");
                StatusCode::INTERNAL_SERVER_ERROR
            }
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
    #[error(transparent)]
    SerdeConversionError(#[from] serde_json::Error),
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
            Self::SerdeConversionError(e) => {
                error!("serde conversion error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
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

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to Create Account")]
    FailedToCreateAccount,
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    SessionError(#[from] tower_sessions::session::Error),
    #[error(transparent)]
    Argon2Error(#[from] argon2::Error),
    #[error(transparent)]
    HashingError(#[from] argon2::password_hash::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    RedisErr(#[from] tower_sessions_redis_store::fred::error::Error),
    #[error(transparent)]
    UuidConersionError(#[from] uuid::Error),
    #[error(transparent)]
    PasswordParseError(#[from] argon2::password_hash::phc::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::SessionError(e) => {
                error!("Session Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Argon2Error(e) => {
                error!("Argon2 Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Other(e) => {
                error!("Unexpected error {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::HashingError(e) => {
                error!("Hashing Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::ValidationError(e) => {
                error!("Error when validating: {e:?}");
                StatusCode::UNPROCESSABLE_ENTITY
            }
            Self::SqlxError(e) => {
                error!("Sqlx Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::RedisErr(e) => {
                error!("Redis Err: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::FailedToCreateAccount => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UuidConersionError(e) => {
                error!("Uuid Conersion Error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::PasswordParseError(e) => {
                error!("Error when parsing password: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status.into_response()
    }
}

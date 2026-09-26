use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use crate::models::db::schema::{SeriesEntity, TitleStatus, UserEntity};
use crate::models::errors::models::DatabaseError;

/// UserResponse DTO
/// that will be returned
/// After the creation of a logged-in user
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub avatar_url: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserEntity> for CreateUserResponse {
    type Error = DatabaseError;
    fn try_from(value: UserEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            username: value.username,
            display_name: value.display_name,
            email: value.email,
            avatar_url: value.avatar_url.unwrap_or_default(),
            created_at: value.created_at,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct NovelQueryResponse {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub series_type_slug: Option<String>,
    pub rating: Option<BigDecimal>,
    pub release_date: Option<NaiveDate>,
    pub status: Option<TitleStatus>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<SeriesEntity> for NovelQueryResponse {
    fn from(value: SeriesEntity) -> Self {
        Self {
            title: value.title,
            slug: value.slug,
            description: value.description,
            series_type_slug: value.series_type_slug,
            rating: value.rating.unwrap_or_default().into(),
            release_date: value.release_date.unwrap_or_default().into(),
            status: Some(value.status),
            updated_at: value.updated_at,
        }
    }
}

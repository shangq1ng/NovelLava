use crate::models::db::schema::{ChaptersEntity, ReviewsEntity, SeriesEntity, TitleStatus, UserEntity};
use crate::models::errors::models::DatabaseError;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

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

#[derive(Debug, Serialize)]
pub struct ChapterDTO {
    pub slug: String,
    pub title: String,
    pub number: Option<BigDecimal>,
    pub s3_path: String,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ChaptersEntity> for ChapterDTO {
    fn from(entity: ChaptersEntity) -> Self {
        Self {
            slug: entity.slug,
            title: entity.title,
            number: entity.number,
            s3_path: entity.s3_path,
            updated_at: entity.updated_at,
        }
    }
}

// DTOs so users can search other users by their username
// Without revealing those searched user's private information like emails, etc.
// without their consent.
#[derive(Debug, Serialize)]
pub struct RandomUserDTO {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewDTO {
    pub username: Option<String>,
    pub parent_id: Option<Uuid>,
    pub child_id: Option<Uuid>,
    pub novel_id: Uuid,
    pub rating: Option<BigDecimal>,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ReviewDTO {
    pub id: Uuid,
    pub username: String,
    pub novel_id: Uuid,
    pub parent_id: Uuid,
    pub content: String,
    pub child_id: Option<Uuid>,
    pub rating: BigDecimal,
    pub created_at: DateTime<Utc>,
}

impl From<ReviewsEntity> for ReviewDTO {
    fn from(entity: ReviewsEntity) -> Self {
        Self {
            id: entity.id,
            username: entity.username,
            novel_id: entity.novel_id,
            parent_id: entity.parent_id,
            child_id: entity.child_id,
            content: entity.content,
            rating: entity.rating.unwrap_or_default(),
            created_at: entity.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllReviews {
    pub novel: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ReviewResponseDTO {
    pub id: Uuid,
    pub username: String,
    pub novel_id: Uuid,
    pub parent_id: Uuid,
    pub content: String,
    pub rating: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub child_id: Option<Uuid>,
    pub child_username: String,
    pub child_content: String,
    pub child_created_at: DateTime<Utc>,
}

impl From<ReviewsEntity> for ReviewResponseDTO {
    fn from(entity: ReviewsEntity) -> Self {
        Self {
            // Parent fields
            id: entity.id,
            username: entity.username.clone(),
            novel_id: entity.novel_id,
            parent_id: entity.parent_id,
            content: entity.content.clone(),
            rating: entity.rating.unwrap_or_default(),
            created_at: entity.created_at.clone(),

            // Child fields
            child_id: entity.child_id.map(|e| return e),
            child_username: entity.username,
            child_content: entity.content,
            child_created_at: entity.created_at,
        }
    }
}

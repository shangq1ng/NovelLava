use crate::models::errors::models::DatabaseError;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::Uuid;
use sqlx::types::chrono::{DateTime, Utc};

/// User Entity
/// Maps 1:1 to our table 'users'
#[derive(Debug, FromRow)]
pub struct UserEntity {
    pub id: Uuid,
    pub sub: Option<String>,
    pub provider: String,
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}
/// Novel Entity
/// Which maps 1:1 to our table 'novels'
#[derive(Debug, FromRow)]
pub struct NovelEntity {
    pub id: Uuid,
    pub author: String,
    pub title: String,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub storage_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

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
    pub author: String,
    pub cover_path: String,
    pub description: String,
    pub tags: String,
}

impl From<NovelEntity> for NovelQueryResponse {
    fn from(value: NovelEntity) -> Self {
        Self {
            title: value.title,
            author: value.author,
            cover_path: value.cover_path.unwrap_or_default(),
            description: value.description.unwrap_or_default(),
            tags: value.tags.unwrap_or_default(),
        }
    }
}

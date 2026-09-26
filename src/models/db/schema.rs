use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, NaiveDate, Utc};
use sqlx::types::{BigDecimal, Uuid};
use sqlx::{FromRow, Type};

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
#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "title_status")]
pub enum TitleStatus {
    OnGoing,
    Cancelled,
    Hiatus,
    Completed,
    Dropped,
}

#[derive(Debug, FromRow)]
pub struct SeriesEntity {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub series_type_slug: Option<String>,
    pub rating: Option<BigDecimal>,
    pub release_date: Option<NaiveDate>,
    pub status: TitleStatus,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct SeriesAuthors {
    pub series_id: Uuid,
    pub author_slug: String,
    pub role: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct SeriesGenres {
    series_id: Uuid,
    genre_slug: String,
}

#[derive(Debug, FromRow)]
pub struct SeriesTags {
    pub series_id: Uuid,
    pub tag_slug: String,
}
#[derive(Debug, FromRow)]
pub struct AuthorsEntity {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct GenresEntity {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct TagsEntity {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct SeriesTypeEntity {
    pub slug: String,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct ChaptersEntity {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub number: Option<i64>,
    pub s3_path: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct CoversEntity {
    pub s3_path: String,
    pub content_type: Option<String>,
}

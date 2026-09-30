use crate::handlers::s3::services::upload_avatar;
use crate::models::auth::models::{UpateUserDTO, User};
use crate::models::config::models::AppConfig;
use crate::models::db::models::{ChapterDTO, RandomUserDTO};
use crate::models::db::schema::{ChaptersEntity, UserEntity};
use crate::models::errors::models::DatabaseError;
use axum::extract::{Path, State};
use axum::{Form, Json};
use axum_extra::extract::Multipart;
use tower_sessions::Session;
use uuid::Uuid;

pub async fn get_chapter_title(
    config: &AppConfig,
    title: &str,
) -> Result<Json<ChapterDTO>, DatabaseError> {
    if !title.is_empty() {
        let result = sqlx::query_as!(
            ChaptersEntity,
            "SELECT id, title, slug, number, updated_at, created_at, s3_path FROM chapters WHERE title = $1", title
        ).fetch_optional(&config.database.db).await?;
        match result {
            Some(chapter) => {
                let res: ChapterDTO = chapter.into();
                Ok(Json(res))
            }
            None => Err(DatabaseError::NotFound),
        }
    } else {
        Err(DatabaseError::NotFound)
    }
}

pub async fn update_display_name(
    State(config): State<AppConfig>,
    session: Session,
    Form(payload): Form<UpateUserDTO>,
) -> Result<Json<User>, DatabaseError> {
    let id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;
    if payload.display_name.is_some() {
        let query = sqlx::query_as!(
            UserEntity,
            "UPDATE users SET display_name = $1 WHERE id = $2 RETURNING *;",
            payload.display_name,
            id
        )
        .fetch_optional(&config.database.db)
        .await?;

        match query {
            Some(usr) => {
                let res = usr.into();
                Ok(Json(res))
            }
            None => Err(DatabaseError::NotFound),
        }
    } else {
        Err(DatabaseError::NotFound)
    }
}

pub async fn find_users_by_username(
    State(config): State<AppConfig>,
    Path(payload): Path<String>,
) -> Result<Json<RandomUserDTO>, DatabaseError> {
    if !payload.is_empty() {
        let query = sqlx::query!(
            "SELECT username, display_name, bio, avatar_url FROM users WHERE username = $1;",
            payload,
        )
        .fetch_optional(&config.database.db)
        .await?;
        match query {
            Some(usr) => {
                let res = RandomUserDTO {
                    username: usr.username,
                    display_name: usr.display_name,
                    avatar_url: usr.avatar_url.unwrap_or_default(),
                    bio: usr.bio,
                };
                Ok(Json(res))
            }
            None => Err(DatabaseError::NotFound),
        }
    } else {
        Err(DatabaseError::NotFound)
    }
}

pub async fn update_bio(
    State(config): State<AppConfig>,
    session: Session,
    Form(payload): Form<UpateUserDTO>,
) -> Result<Json<User>, DatabaseError> {
    let id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;
    if payload.bio.is_some() {
        let query = sqlx::query_as!(
            UserEntity,
            "UPDATE users SET bio = $1 WHERE id = $2 RETURNING *;",
            payload.bio,
            id
        )
        .fetch_optional(&config.database.db)
        .await?;

        match query {
            Some(usr) => {
                let res = usr.into();
                Ok(Json(res))
            }
            None => Err(DatabaseError::NotFound),
        }
    } else {
        Err(DatabaseError::NotFound)
    }
}

pub async fn update_avatar(
    State(config): State<AppConfig>,
    session: Session,
    mut payload: Multipart,
) -> Result<Json<User>, DatabaseError> {
    let id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;

    while let Some(field) = payload.next_field().await? {
        let avatar = field.bytes().await?;
        if !avatar.is_empty() {
            if upload_avatar(&config, id, avatar).await.is_ok() {
                let avatar_url = format!("novellava/avatars/{}.png", id);
                let query = sqlx::query_as!(
                    UserEntity,
                    "UPDATE users SET avatar_url = $1 WHERE id = $2 RETURNING *;",
                    avatar_url,
                    id
                )
                    .fetch_optional(&config.database.db)
                    .await?;

                return match query {
                    Some(usr) => {
                        let res = usr.into();
                        Ok(Json(res))
                    }
                    None => Err(DatabaseError::NotFound)
                }
            }
        }
    }
    Err(DatabaseError::NotFound)
}
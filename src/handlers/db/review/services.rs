use crate::models::config::models::AppConfig;
use crate::models::db::models::{CreateReviewDTO, GetAllReviews, ReviewDTO, ReviewResponseDTO};
use crate::models::db::schema::ReviewsEntity;
use crate::models::errors::models::DatabaseError;
use axum::extract::{Query, State};
use axum::{Form, Json};
use sqlx::query_as;
use tower_sessions::Session;
use uuid::Uuid;
/// Creates a new review for a novel
pub async fn create_review(
    State(state): State<AppConfig>,
    session: Session,
    Form(payload): Form<CreateReviewDTO>,
) -> Result<Json<ReviewDTO>, DatabaseError> {
    let content = payload.content.trim();
    let session_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;
    let query = query_as!(
            ReviewsEntity,
            "INSERT INTO reviews(novel_id, content, rating, id, username) VALUES ($1, $2, $3, $4, $5) RETURNING *;",
            payload.novel_id,
            content,
            payload.rating,
            session_id,
            payload.username
        ).fetch_one(&state.database.db).await?;

    let result: ReviewDTO = query.into();
    Ok(Json(result))
}

/// Replies on novel reviews
pub async fn reply_review(
    State(state): State<AppConfig>,
    session: Session,
    Form(payload): Form<CreateReviewDTO>,
) -> Result<Json<ReviewDTO>, DatabaseError> {
    let content = payload.content.trim();
    let session_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;
    let query = query_as!(
            ReviewsEntity,
            "INSERT INTO reviews(id, username, novel_id, child_id, content) VALUES ($1, $2, $3, $4, $5) RETURNING *;",
            session_id,
            payload.username,
            payload.novel_id,
            payload.child_id,
            content,
        ).fetch_one(&state.database.db).await?;

    let result: ReviewDTO = query.into();
    Ok(Json(result))
}

/// Edit/Update review/comment
pub async fn edit_review(
    State(state): State<AppConfig>,
    session: Session,
    Form(payload): Form<CreateReviewDTO>,
) -> Result<Json<ReviewDTO>, DatabaseError> {
    let session_id = session
        .get::<Uuid>("user_id")
        .await
        .map_err(DatabaseError::TowerSessionError)?
        .ok_or(DatabaseError::NotFound)?;
    let query = sqlx::query_as!(
        ReviewsEntity,
        "UPDATE reviews SET content = $1 WHERE novel_id = $2 AND id = $3 AND parent_id = $4 RETURNING *;",
        payload.content,
        payload.novel_id,
        session_id,
        payload.parent_id
    ).fetch_one(&state.database.db).await?;

    let result: ReviewDTO = query.into();

    Ok(Json(result))
}

pub async fn get_all_reviews(
    State(state): State<AppConfig>,
    Query(payload): Query<GetAllReviews>,
) -> Result<Json<Vec<ReviewDTO>>, DatabaseError> {
    let query = query_as!(
        ReviewsEntity,
        "SELECT id, content, novel_id, parent_id, child_id, rating, username, created_at FROM reviews WHERE novel_id = $1;",
        payload.novel
    ).fetch_all(&state.database.db).await?;

    let result: Vec<ReviewDTO> = query.into_iter().map(|r| r.into()).collect();
    Ok(Json(result))
}

pub async fn get_all_child_reviews(
    State(state): State<AppConfig>,
    Query(payload): Query<GetAllReviews>,
) -> Result<Json<Vec<ReviewResponseDTO>>, DatabaseError> {
    let query = sqlx::query!(
        r#"SELECT
    parent.id,
    parent.username,
    parent.novel_id,
    parent.parent_id   AS thread_id,
    parent.content     AS parent_content,
    parent.rating      AS parent_rating,
    parent.created_at  AS parent_created_at,
    child.id           AS child_id,
    child.username     AS child_username,
    child.content      AS child_content,
    child.rating       AS child_rating,
    child.created_at   AS child_created_at
    FROM reviews parent
    LEFT JOIN reviews child
       ON child.child_id  = parent.parent_id
      AND child.novel_id  = parent.novel_id
    WHERE parent.novel_id = $1
    AND parent.child_id IS NULL
    ORDER BY parent.created_at, child.created_at;"#,
        payload.novel
    )
    .fetch_all(&state.database.db)
    .await?;

    let result: Vec<ReviewResponseDTO> = query
        .into_iter()
        .map(|r| {
            ReviewResponseDTO {
                // Parent fields
                id: r.id,
                username: r.username,
                novel_id: r.novel_id,
                parent_id: r.thread_id,
                content: r.parent_content,
                rating: r.parent_rating.unwrap_or_default(),
                created_at: r.parent_created_at,

                // Child fields
                child_username: r.child_username,
                child_content: r.child_content,
                child_id: Option::from(r.child_id),
                child_created_at: r.child_created_at,
            }
        })
        .collect();

    Ok(Json(result))
}

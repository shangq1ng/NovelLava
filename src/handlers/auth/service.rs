use std::str::FromStr;
use crate::handlers::auth::helpers::hash_password;
use crate::models::config::models::AppConfig;
use crate::models::errors::models::AuthError;
use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use tower_sessions::Session;
use tracing::error;
use validator::Validate;
use uuid::Uuid;
use crate::models::auth::models::{SignUpRequest, SignUpUserIdentity};
use crate::models::db::models::UserEntity;

pub async fn sign_up(
    State(state): State<AppConfig>,
    session: Session,
    Json(payload): Json<SignUpRequest>,
) -> Result<impl IntoResponse, AuthError> {
    if let Err(e) = payload.validate() {
        error!("Crooked payload: {}", e);
        return Err(AuthError::ValidationError(e));
    };

    let hashed_password = hash_password(payload.password.as_str()).await?;
    let provider = String::from("SignIn");
    let user = sqlx::query!(
        "INSERT INTO users(provider, username, display_name, email, password)
        VALUES($1, $2, $3, $4, $5) RETURNING id;",
        provider,
        payload.username,
        payload.display_name,
        payload.email,
        hashed_password
    )
    .fetch_optional(&state.database.db)
    .await?;

    match user {
        Some(usr) => {
            tracing::info!("Successfully created user");
            session.insert("user_id", &usr.id).await?;
            session.cycle_id().await?;
            Ok(Redirect::to("/"))
        }
        None => {
            error!("Failed to create user account");
            Err(AuthError::FailedToCreateAccount)
        }
    }
}

pub async fn home() -> Result<String, AuthError> {
    Ok(
        "Hello World".to_string(), // TODO: Change it later
    )
}

pub async fn me(
    State(state): State<AppConfig>,
    session: Session,
) -> Result<Json<SignUpUserIdentity>, AuthError> {
    let session = session.get::<String>("user_id").await
        .map_err(|e| AuthError::SessionError(e))?
        .ok_or(AuthError::NotFound)?;
    let user_id = Uuid::from_str(&session)?;
    if !user_id.is_nil() {
        let user = sqlx::query_as!(
            UserEntity,
            "SELECT id, sub, username, display_name, email, avatar_url, provider, password, created_at FROM users WHERE id = $1",
            user_id
        ).fetch_optional(&state.database.db).await?;

        match user {
            Some(user) => {
                let response = user.into();
                Ok(Json(response))
            }
            None => Err(AuthError::NotFound),
        }
    } else {
        Err(AuthError::NotFound)
    }
}
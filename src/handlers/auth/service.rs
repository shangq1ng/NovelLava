use crate::handlers::auth::helpers::{hash_password, verify_password, TemplateRender};
use crate::models::auth::models::{SignInPage, SignUpPage, SignUpRequest, User};
use crate::models::config::models::AppConfig;
use crate::models::db::models::UserEntity;
use crate::models::errors::models::AuthError;
use axum::{Form, Json};
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use std::str::FromStr;
use tower_sessions::Session;
use tracing::error;
use uuid::Uuid;
use validator::Validate;

pub async fn signin_page() -> Response {
    SignInPage { error_msg: None }.into_html()
}
pub async fn signup_page() -> Response {
    SignUpPage { error_msg: None }.into_html()
}
pub async fn sign_up(
    State(state): State<AppConfig>,
    session: Session,
    Form(payload): Form<SignUpRequest>,
) -> Result<impl IntoResponse, AuthError> {
    if let Err(e) = payload.validate() {
        error!("Crooked payload: {}", e);
        return Err(AuthError::ValidationError(e));
    };
    let username = payload.username.trim().to_lowercase();
    let hashed_password = hash_password(payload.password.as_str())?;
    let provider = String::from("SignIn");
    let user = sqlx::query!(
        "INSERT INTO users(provider, username, display_name, email, password)
        VALUES($1, $2, $3, $4, $5) RETURNING id;",
        provider,
        username,
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
            Ok(Redirect::to("/users/me"))
        }
        None => {
            error!("Failed to create user account");
            Err(AuthError::FailedToCreateAccount)
        }
    }
}

pub async fn sign_in(
    State(state): State<AppConfig>,
    session: Session,
    Form(payload): Form<SignUpRequest>,
) -> Result<impl IntoResponse, AuthError> {
    if !payload.username.trim().to_lowercase().is_empty() && !payload.password.trim().is_empty() {
        let user = sqlx::query_as!(
            UserEntity,
            "SELECT id, sub, created_at, provider, username, display_name, avatar_url, email, password FROM users WHERE username = $1",
            payload.username,
        ).fetch_optional(&state.database.db).await?;
        match user {
            Some(user) => {
                let valid = verify_password(&payload.password, user.password.unwrap_or_default().as_str())?;
                if !valid {
                    Err(AuthError::NotFound)
                } else {
                    session.cycle_id().await?;
                    Ok(Redirect::to("/users/me"))
                }
            }
            None => Err(AuthError::NotFound),
        }
    } else {
        Err(AuthError::NotFound)
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
) -> Result<Json<User>, AuthError> {
    let session = session
        .get::<String>("user_id")
        .await
        .map_err(AuthError::SessionError)?
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

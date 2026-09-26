use crate::handlers::auth::helpers::{TemplateRender, hash_password, verify_password};
use crate::handlers::cache::services::{get_user, insert_user};
use crate::models::auth::models::{SignInPage, SignUpPage, SignUpRequest, User};
use crate::models::config::models::AppConfig;
use crate::models::db::schema::UserEntity;
use crate::models::errors::models::AuthError;
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{Form, Json};
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
            session.cycle_id().await?;
            session.insert("user_id", &usr.id).await?;
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
                let valid = verify_password(
                    &payload.password,
                    user.password.unwrap_or_default().as_str(),
                )?;
                if !valid {
                    Err(AuthError::NotFound)
                } else {
                    session.cycle_id().await?;
                    session.insert("user_id", &user.id).await?;
                    Ok(Redirect::to("/users/me"))
                }
            }
            None => Err(AuthError::NotFound),
        }
    } else {
        Err(AuthError::NotFound)
    }
}

pub async fn me(State(state): State<AppConfig>, session: Session) -> Result<Json<User>, AuthError> {
    let session = session
        .get::<Uuid>("user_id")
        .await
        .map_err(AuthError::SessionError)?
        .ok_or(AuthError::NotFound)?;
    let session_key = session.clone();
    let usr = get_user(&state, session_key.to_string().as_str()).await.unwrap_or_default();
    match usr {
        Some(usr) => Ok(Json(usr)),
        None => {
            let user = sqlx::query_as!(
            UserEntity,
            "SELECT id, sub, username, display_name, email, avatar_url, provider, password, created_at FROM users WHERE id = $1",
            session
        ).fetch_optional(&state.database.db).await?;

            match user {
                Some(user) => {
                    let response = user.into();
                    let is_success = insert_user(&state, &response, session_key.to_string().as_str()).await.unwrap_or(Some(false));
                    match is_success {
                        Some(true) => {
                            // do nothing
                        },
                        Some(false) => {
                            error!("Failed to insert user")
                        }
                        None => {
                            error!("It didn't insert, nor did it failed to insert. Something's wrong with the logic itself or redis is not up for connections.")
                        },
                    }
                    Ok(Json(response))
                }
                None => Err(AuthError::NotFound),
            }
        }
    }
}

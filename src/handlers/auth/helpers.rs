use crate::models::errors::models::AuthError;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use askama::Template;
use axum::response::{Html, IntoResponse, Response};

pub trait TemplateRender {
    fn into_html(self) -> Response;
}

impl<T: Template> TemplateRender for T {
    fn into_html(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => "Failed to render html".into_response(),
        }
    }
}
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let argon2 = argon2::Argon2::default();
    let hash = argon2.hash_password(password.as_bytes())?.to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let argon2 = argon2::Argon2::default();

    let parsed = PasswordHash::new(hash)?;

    match argon2.verify_password(password.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(e) => Err(AuthError::HashingError(e)),
    }
}

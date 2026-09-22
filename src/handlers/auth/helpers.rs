use crate::models::errors::models::AuthError;
use argon2::PasswordHasher;

pub async fn hash_password(password: &str) -> Result<String, AuthError> {
    let argon2 = argon2::Argon2::default();
    let hash = argon2.hash_password(&password.as_bytes())?.to_string();
    Ok(hash)
}

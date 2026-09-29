use crate::models::errors::models::AuthError;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
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

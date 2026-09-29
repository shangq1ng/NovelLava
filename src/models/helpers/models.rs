use crate::handlers::helpers::services::{AnonymousUser, AuthenticatedUser};
use axum::http::Request;
use std::net::IpAddr;
use tower_governor::GovernorError;
use tower_governor::key_extractor::KeyExtractor;
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum UserType {
    Anon(IpAddr),
    LoggedUser(Uuid),
}
#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub struct SmartExtractor;

impl KeyExtractor for SmartExtractor {
    type Key = UserType;
    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        if let Some(v) = req
            .extensions()
            .get::<AuthenticatedUser>()
            .map(|user| user.0)
        {
            return Ok(UserType::LoggedUser(v));
        };

        if let Some(v) = req.extensions().get::<AnonymousUser>().map(|user| user.0 ) {
            Ok(UserType::Anon(v))
        } else {
            Err(GovernorError::UnableToExtractKey)
        }
    }
}

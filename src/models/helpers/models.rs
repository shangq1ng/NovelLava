use crate::handlers::helpers::services::AuthenticatedUser;
use axum::http::Request;
use std::net::{IpAddr, SocketAddr};
use tower_governor::GovernorError;
use tower_governor::key_extractor::KeyExtractor;
use uuid::Uuid;

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
pub enum RateLimit {
    Anon(IpAddr),
    User(Uuid),
}
#[derive(Clone, Debug)]
pub struct SmartExtractor;

impl KeyExtractor for SmartExtractor {
    type Key = RateLimit;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        if let Some(val) = req
            .extensions()
            .get::<AuthenticatedUser>()
            .map(|AuthenticatedUser(id)| *id)
        {
            Ok(RateLimit::User(val))
        } else {
            let addr = req
                .extensions()
                .get::<SocketAddr>()
                .ok_or(GovernorError::UnableToExtractKey)?;
            Ok(RateLimit::Anon(addr.ip()))
        }
    }
}

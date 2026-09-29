use crate::handlers::helpers::services::AuthenticatedUser;
use axum::extract::ConnectInfo;
use axum::http::Request;
use std::net::{IpAddr, SocketAddr};
use tower_governor::GovernorError;
use tower_governor::key_extractor::KeyExtractor;
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum UserType {
    Anon(IpAddr),
    User(Uuid),
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
            Ok(UserType::User(v))
        } else {
            let addr = req
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .ok_or(GovernorError::UnableToExtractKey)?;
            Ok(UserType::Anon(addr.ip()))
        }
    }
}

use axum::extract::FromRef;
use oauth2::{EndpointNotSet, EndpointSet};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::models::db::models::UserEntity;

pub type Client = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub client: Client,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub state: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserResponse {
    // google user responses (eg; sub, exp, picture, name, etc)
}

#[derive(Debug, Deserialize)]
pub struct DiscordUserResponse {
    // discord user responses (eg; avatar_url, username, guild, etc)
}

impl FromRef<AuthConfig> for Client {
    fn from_ref(config: &AuthConfig) -> Self {
        config.client.clone()
    }
}


#[derive(Deserialize, Validate, Debug)]
pub struct SignUpRequest {
    pub username: String,
    pub display_name: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 8))]
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct SignUpUserIdentity {
    pub username: String,
    pub display_name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
}

impl From<UserEntity> for SignUpUserIdentity {
    fn from(entity: UserEntity) -> Self {
        Self {
            username: entity.username,
            display_name: entity.display_name,
            email: entity.email,
            avatar_url: entity.avatar_url,
        }
    }
}
use axum::extract::FromRef;
use oauth2::{EndpointNotSet, EndpointSet};
use serde::Deserialize;
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

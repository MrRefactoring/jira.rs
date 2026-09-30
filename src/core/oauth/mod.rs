//! Atlassian OAuth 2.0, for Jira Cloud (3LO) and for a Data Center instance's own provider.

mod helpers;
mod manager;
mod server;
mod types;

pub use helpers::{
    AuthorizationUrlParams, ExchangeCodeParams, RefreshTokenParams, exchange_authorization_code,
    generate_authorization_url, get_accessible_resources, parse_callback_url, refresh_oauth2_token,
};
pub use server::{
    ServerAuthorizationUrlParams, ServerExchangeCodeParams, ServerOAuth2Scope, ServerRefreshTokenParams,
    exchange_server_authorization_code, generate_server_authorization_url, refresh_server_oauth2_token,
};
pub use types::{AccessibleResource, CallbackParams, TokenRefreshEvent, TokenResponse};

pub(crate) use manager::OAuth2Manager;

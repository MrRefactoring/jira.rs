use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// The token endpoint's answer, in this crate's vocabulary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The rotated refresh token, present when `offline_access` was requested.
    ///
    /// Persist it — Atlassian invalidates the one that was sent.
    #[serde(rename = "refresh_token", default)]
    pub refresh_token: Option<String>,
    /// Access-token lifetime in seconds, as returned by Atlassian. Typically 3600.
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
    /// Space-separated granted scopes.
    #[serde(default)]
    pub scope: String,
    /// Always `bearer`.
    #[serde(rename = "token_type", default)]
    pub token_type: String,
}

impl TokenResponse {
    /// When this access token expires, counted from now.
    pub fn expires_at(&self) -> SystemTime {
        SystemTime::now() + Duration::from_secs(self.expires_in)
    }
}

/// An entry from `GET /oauth/token/accessible-resources`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleResource {
    /// The cloud id — this is what `cloud_id` expects.
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// Site URL, e.g. `https://your-domain.atlassian.net`.
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "avatarUrl", default)]
    pub avatar_url: String,
}

/// Handed to the refresh hook after every successful refresh.
#[derive(Debug, Clone)]
pub struct TokenRefreshEvent {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: SystemTime,
}

/// What the redirect callback carried, once it was checked.
#[derive(Debug, Clone)]
pub struct CallbackParams {
    /// The authorization code, ready for [`exchange_authorization_code`](super::exchange_authorization_code).
    pub code: String,
    /// The `state` that came back, already verified against the expected one.
    pub state: String,
}

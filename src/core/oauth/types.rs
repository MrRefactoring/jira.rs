use std::fmt;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// The token endpoint's answer, in this crate's vocabulary.
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "access_token")]
    /// The bearer token to send from now on.
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

impl fmt::Debug for TokenResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenResponse")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &self.refresh_token.as_deref().map(|_| "<redacted>"))
            .field("expires_in", &self.expires_in)
            .field("scope", &self.scope)
            .field("token_type", &self.token_type)
            .finish()
    }
}

const LONGEST_LIFETIME: Duration = Duration::from_secs(365 * 24 * 60 * 60);

impl TokenResponse {
    /// When this access token expires, counted from now.
    pub fn expires_at(&self) -> SystemTime {
        SystemTime::now() + Duration::from_secs(self.expires_in).min(LONGEST_LIFETIME)
    }
}

/// An entry from `GET /oauth/token/accessible-resources`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibleResource {
    /// The cloud id — this is what `cloud_id` expects.
    pub id: String,
    #[serde(default)]
    /// The site's display name.
    pub name: String,
    /// Site URL, e.g. `https://your-domain.atlassian.net`.
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    /// The scopes the token was granted on this site.
    pub scopes: Vec<String>,
    #[serde(rename = "avatarUrl", default)]
    /// The site's avatar.
    pub avatar_url: String,
}

/// Handed to the refresh hook after every successful refresh.
#[derive(Clone)]
pub struct TokenRefreshEvent {
    /// The new access token.
    pub access_token: String,
    /// The rotated refresh token, when one was issued. Persist it: the previous one is dead.
    pub refresh_token: Option<String>,
    /// When the new access token expires.
    pub expires_at: SystemTime,
}

impl fmt::Debug for TokenRefreshEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenRefreshEvent")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &self.refresh_token.as_deref().map(|_| "<redacted>"))
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

/// What the redirect callback carried, once it was checked.
#[derive(Clone)]
pub struct CallbackParams {
    /// The authorization code, ready for [`exchange_authorization_code`](super::exchange_authorization_code).
    pub code: String,
    /// The `state` that came back, already verified against the expected one.
    pub state: String,
}

impl fmt::Debug for CallbackParams {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("CallbackParams").field("code", &"<redacted>").field("state", &self.state).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn answering_with(expires_in: u64) -> TokenResponse {
        TokenResponse {
            access_token: "token".to_owned(),
            refresh_token: None,
            expires_in,
            scope: String::new(),
            token_type: "bearer".to_owned(),
        }
    }

    #[test]
    fn counts_the_lifetime_the_token_endpoint_gave_from_now() {
        let before = SystemTime::now();
        let expires_at = answering_with(3600).expires_at();

        let lifetime = expires_at.duration_since(before).expect("an hour from now is after now");

        assert!(lifetime >= Duration::from_secs(3600));
        assert!(lifetime < Duration::from_secs(3600 + 60));
    }

    #[test]
    fn holds_a_lifetime_no_instant_could_hold_at_a_year() {
        let before = SystemTime::now();
        let expires_at = answering_with(u64::MAX).expires_at();

        let lifetime = expires_at.duration_since(before).expect("a year from now is after now");

        assert!(lifetime >= LONGEST_LIFETIME);
        assert!(lifetime < LONGEST_LIFETIME + Duration::from_secs(60));
    }
}

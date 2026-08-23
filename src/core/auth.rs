use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use crate::core::error::{Error, Result};
use crate::core::oauth::TokenRefreshEvent;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Supplies a bearer token per request, for callers who mint their own.
pub trait TokenProvider: Send + Sync + 'static {
    fn token(&self) -> BoxFuture<'_, Result<String>>;
}

impl<F, Fut> TokenProvider for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<String>> + Send + 'static,
{
    fn token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(self())
    }
}

/// Called after every OAuth 2.0 refresh.
///
/// Persist the rotated refresh token here — Atlassian invalidates the one that was sent, so the previous value is
/// dead the moment this runs.
pub trait TokenRefreshHook: Send + Sync + 'static {
    fn on_token_refresh(&self, event: TokenRefreshEvent) -> BoxFuture<'_, ()>;
}

impl<F, Fut> TokenRefreshHook for F
where
    F: Fn(TokenRefreshEvent) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    fn on_token_refresh(&self, event: TokenRefreshEvent) -> BoxFuture<'_, ()> {
        Box::pin(self(event))
    }
}

/// Atlassian OAuth 2.0 (3LO), against Jira Cloud.
///
/// Either hand over an `access_token` and manage its lifetime yourself, or hand over the full refresh credential set
/// and let the client refresh on its own. A partial credential set is always a mistake — it looks configured, then
/// fails on the first refresh — and is rejected when the client is built.
#[derive(Clone, Default)]
pub struct OAuth2Config {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    /// When the access token expires. The client refreshes a minute ahead of it.
    pub expires_at: Option<SystemTime>,
    /// Atlassian cloud id. When set, the `accessible-resources` lookup is skipped.
    pub cloud_id: Option<String>,
    /// Site URL, e.g. `https://your-domain.atlassian.net`, used to pick a cloud id when the token reaches several.
    pub site_url: Option<String>,
    pub on_token_refresh: Option<Arc<dyn TokenRefreshHook>>,
}

impl fmt::Debug for OAuth2Config {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OAuth2Config")
            .field("access_token", &redacted(self.access_token.as_deref()))
            .field("refresh_token", &redacted(self.refresh_token.as_deref()))
            .field("client_id", &self.client_id)
            .field("client_secret", &redacted(self.client_secret.as_deref()))
            .field("expires_at", &self.expires_at)
            .field("cloud_id", &self.cloud_id)
            .field("site_url", &self.site_url)
            .finish_non_exhaustive()
    }
}

/// OAuth 2.0 against a Data Center instance's own provider.
///
/// A separate strategy from [`OAuth2Config`] rather than a flag on it, because the two differ in what they talk to.
/// Cloud 3LO tokens are minted by `auth.atlassian.com` and accepted only through the Atlassian gateway, so the base
/// URL is derived from a cloud id; a Data Center instance is its own authorization server and its own API host, so
/// `host` is required and is the only address involved.
#[derive(Clone, Default)]
pub struct OAuth2ServerConfig {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    /// The redirect URI the incoming application link was registered with. The provider validates it on refresh too.
    pub redirect_uri: Option<String>,
    pub expires_at: Option<SystemTime>,
    pub on_token_refresh: Option<Arc<dyn TokenRefreshHook>>,
}

impl fmt::Debug for OAuth2ServerConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OAuth2ServerConfig")
            .field("access_token", &redacted(self.access_token.as_deref()))
            .field("refresh_token", &redacted(self.refresh_token.as_deref()))
            .field("client_id", &self.client_id)
            .field("client_secret", &redacted(self.client_secret.as_deref()))
            .field("redirect_uri", &self.redirect_uri)
            .field("expires_at", &self.expires_at)
            .finish_non_exhaustive()
    }
}

/// How the client proves who it is.
#[derive(Clone)]
#[non_exhaustive]
pub enum Auth {
    /// Cloud pairs an account address with an API token, Data Center a username with a password. The wire format is
    /// the same; only the two halves differ.
    Basic {
        username: String,
        password: String,
    },
    /// A personal access token, which is how Data Center 8.14 and later prefer to be addressed.
    Bearer {
        token: String,
    },
    /// A bearer token minted per request.
    BearerProvider(Arc<dyn TokenProvider>),
    OAuth2(OAuth2Config),
    OAuth2Server(OAuth2ServerConfig),
}

impl Auth {
    /// Cloud: an Atlassian account address and an API token minted for it.
    pub fn api_token(email: impl Into<String>, api_token: impl Into<String>) -> Self {
        Auth::Basic { username: email.into(), password: api_token.into() }
    }

    /// Data Center: a local account name and its password.
    pub fn password(username: impl Into<String>, password: impl Into<String>) -> Self {
        Auth::Basic { username: username.into(), password: password.into() }
    }

    /// A personal access token, or any other bearer credential.
    pub fn bearer(token: impl Into<String>) -> Self {
        Auth::Bearer { token: token.into() }
    }

    pub fn oauth2(config: OAuth2Config) -> Self {
        Auth::OAuth2(config)
    }

    pub fn oauth2_server(config: OAuth2ServerConfig) -> Self {
        Auth::OAuth2Server(config)
    }

    /// Rejects a credential set that cannot work, so the mistake surfaces before the first request rather than as a
    /// puzzling 401 an hour later.
    pub fn validate(&self) -> Result<()> {
        match self {
            Auth::Basic { username, password } => {
                if username.trim().is_empty() {
                    return Err(Error::config("Basic authentication needs an account address or username."));
                }

                if password.is_empty() {
                    return Err(Error::config("Basic authentication needs an API token or password."));
                }

                Ok(())
            }
            Auth::Bearer { token } => {
                if token.trim().is_empty() {
                    return Err(Error::config("Bearer authentication needs a token."));
                }

                Ok(())
            }
            Auth::BearerProvider(_) => Ok(()),
            Auth::OAuth2(config) => validate_oauth2(
                config.access_token.as_deref(),
                &[
                    ("refreshToken", config.refresh_token.as_deref()),
                    ("clientId", config.client_id.as_deref()),
                    ("clientSecret", config.client_secret.as_deref()),
                ],
                "OAuth 2.0",
            ),
            Auth::OAuth2Server(config) => validate_oauth2(
                config.access_token.as_deref(),
                &[
                    ("refreshToken", config.refresh_token.as_deref()),
                    ("clientId", config.client_id.as_deref()),
                    ("clientSecret", config.client_secret.as_deref()),
                    ("redirectUri", config.redirect_uri.as_deref()),
                ],
                "Data Center OAuth 2.0",
            ),
        }
    }

    /// The `Authorization` header for the credentials that carry one on their own.
    ///
    /// Both OAuth strategies are absent: their header comes from a manager, which refreshes first.
    pub(crate) async fn authorization_header(&self) -> Result<Option<String>> {
        match self {
            Auth::Basic { username, password } => {
                Ok(Some(format!("Basic {}", encode_base64(format!("{username}:{password}").as_bytes()))))
            }
            Auth::Bearer { token } => Ok(Some(format!("Bearer {token}"))),
            Auth::BearerProvider(provider) => Ok(Some(format!("Bearer {}", provider.token().await?))),
            Auth::OAuth2(config) => Ok(config.access_token.as_ref().map(|token| format!("Bearer {token}"))),
            Auth::OAuth2Server(config) => Ok(config.access_token.as_ref().map(|token| format!("Bearer {token}"))),
        }
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Auth::Basic { username, .. } => {
                formatter.debug_struct("Basic").field("username", username).field("password", &"<redacted>").finish()
            }
            Auth::Bearer { .. } => formatter.debug_struct("Bearer").field("token", &"<redacted>").finish(),
            Auth::BearerProvider(_) => formatter.write_str("BearerProvider(<fn>)"),
            Auth::OAuth2(config) => fmt::Debug::fmt(config, formatter),
            Auth::OAuth2Server(config) => fmt::Debug::fmt(config, formatter),
        }
    }
}

fn redacted(value: Option<&str>) -> &'static str {
    if value.is_some() { "<redacted>" } else { "None" }
}

fn validate_oauth2(access_token: Option<&str>, refresh_set: &[(&str, Option<&str>)], label: &str) -> Result<()> {
    let present = refresh_set.iter().filter(|(_, value)| value.is_some()).count();
    let complete = present == refresh_set.len();
    let names = refresh_set.iter().map(|(name, _)| format!("`{name}`")).collect::<Vec<_>>().join(", ");

    if access_token.is_none() && !complete {
        return Err(Error::config(format!(
            "{label} requires either an `accessToken` or a full refresh credential set ({names})."
        )));
    }

    if present > 0 && !complete {
        return Err(Error::config(format!("When using {label} token refresh, {names} must all be provided together.")));
    }

    Ok(())
}

const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Base64 for the Basic auth header.
///
/// Written out rather than pulled in: it is fourteen lines against a dependency in every consumer's tree, and this is
/// the only place the crate encodes anything.
pub(crate) fn encode_base64(input: &[u8]) -> String {
    let mut encoded = String::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        let bytes = [chunk[0], chunk.get(1).copied().unwrap_or(0), chunk.get(2).copied().unwrap_or(0)];
        let triple = (u32::from(bytes[0]) << 16) | (u32::from(bytes[1]) << 8) | u32::from(bytes[2]);

        for index in 0..4 {
            if index <= chunk.len() {
                let position = (triple >> (18 - index * 6)) & 0x3F;

                encoded.push(char::from(BASE64_ALPHABET[position as usize]));
            } else {
                encoded.push('=');
            }
        }
    }

    encoded
}

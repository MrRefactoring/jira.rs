use crate::core::error::Result;
use crate::core::oauth::helpers::{post_form, post_json};
use crate::core::oauth::types::TokenResponse;

/// Data Center is its own authorization server.
///
/// Nothing here goes near `auth.atlassian.com` or `api.atlassian.com`: a self-hosted instance issues its own tokens
/// on its own domain, so every call takes the site as an argument. There is no cloud id and no gateway.
const AUTHORIZE_PATH: &str = "/rest/oauth2/latest/authorize";
pub(crate) const TOKEN_PATH: &str = "/rest/oauth2/latest/token";

/// The four access levels a Data Center incoming link can grant.
///
/// Each implies the ones above it: `Write` includes `Read`, `Admin` includes both, and `SystemAdmin` includes all
/// three.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerOAuth2Scope {
    /// Read the instance's data.
    Read,
    /// Read and write.
    Write,
    /// Read, write and administer projects.
    Admin,
    /// Everything, the instance's own administration included.
    SystemAdmin,
}

impl ServerOAuth2Scope {
    /// The scope as the instance spells it, e.g. `SYSTEM_ADMIN`.
    pub fn as_str(self) -> &'static str {
        match self {
            ServerOAuth2Scope::Read => "READ",
            ServerOAuth2Scope::Write => "WRITE",
            ServerOAuth2Scope::Admin => "ADMIN",
            ServerOAuth2Scope::SystemAdmin => "SYSTEM_ADMIN",
        }
    }
}

pub(crate) fn normalize_host(host: &str) -> &str {
    host.trim_end_matches('/')
}

/// Where to send the user so they can grant access to a Data Center instance.
///
/// The `client_id` and `redirect_uri` are the ones an administrator registered as an incoming application link; a
/// `redirect_uri` that does not match the registration is rejected by the instance rather than by this function.
#[derive(Debug, Clone)]
pub struct ServerAuthorizationUrlParams {
    /// The instance's site URL, e.g. `https://jira.example.com`.
    pub host: String,
    /// The client id of the incoming application link.
    pub client_id: String,
    /// The access levels to ask for.
    pub scopes: Vec<ServerOAuth2Scope>,
    /// Where the instance sends the user back to, exactly as registered on the link.
    pub redirect_uri: String,
    /// Yours to generate and to verify when the callback comes back — it is what stops CSRF on the redirect.
    pub state: String,
}

/// The authorization code to trade in, against a Data Center instance.
#[derive(Debug, Clone)]
pub struct ServerExchangeCodeParams {
    /// The instance's site URL, e.g. `https://jira.example.com`.
    pub host: String,
    /// The client id of the incoming application link.
    pub client_id: String,
    /// The client secret of the incoming application link.
    pub client_secret: String,
    /// The authorization code from the redirect callback.
    pub code: String,
    /// The redirect URI the code was issued for.
    pub redirect_uri: String,
    /// The HTTP client to reach the instance by, so a proxy or a timeout covers the token calls too.
    pub http: Option<reqwest::Client>,
}

/// The refresh credential set, against a Data Center instance.
#[derive(Debug, Clone)]
pub struct ServerRefreshTokenParams {
    /// The instance's site URL, e.g. `https://jira.example.com`.
    pub host: String,
    /// The client id of the incoming application link.
    pub client_id: String,
    /// The client secret of the incoming application link.
    pub client_secret: String,
    /// The refresh token in hand.
    pub refresh_token: String,
    /// Required where the specification does not ask for it: the Data Center provider validates it on the refresh
    /// grant as well, and omitting it earns an `invalid_grant` that names nothing.
    pub redirect_uri: String,
    /// The HTTP client to reach the instance by, so a proxy or a timeout covers the token calls too.
    pub http: Option<reqwest::Client>,
}

/// Build the URL to send the user to so they can grant access to a Data Center instance.
pub fn generate_server_authorization_url(params: &ServerAuthorizationUrlParams) -> String {
    let scopes = params.scopes.iter().map(|scope| scope.as_str()).collect::<Vec<_>>().join(" ");
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &params.client_id)
        .append_pair("scope", &scopes)
        .append_pair("redirect_uri", &params.redirect_uri)
        .append_pair("state", &params.state)
        .append_pair("response_type", "code")
        .finish();

    format!("{}{AUTHORIZE_PATH}?{query}", normalize_host(&params.host))
}

/// Exchange the authorization `code` from the redirect callback for tokens.
///
/// Form-encoded rather than JSON, unlike the Cloud helpers: this endpoint is a Java servlet reading request
/// parameters, and a form body keeps the client secret out of the URL that proxies and access logs record.
pub async fn exchange_server_authorization_code(params: &ServerExchangeCodeParams) -> Result<TokenResponse> {
    let url = format!("{}{TOKEN_PATH}", normalize_host(&params.host));

    post_form(
        params.http.as_ref(),
        &url,
        &[
            ("grant_type", "authorization_code"),
            ("client_id", &params.client_id),
            ("client_secret", &params.client_secret),
            ("code", &params.code),
            ("redirect_uri", &params.redirect_uri),
        ],
    )
    .await
}

/// Refresh an access token against a Data Center instance.
pub async fn refresh_server_oauth2_token(params: &ServerRefreshTokenParams) -> Result<TokenResponse> {
    let url = format!("{}{TOKEN_PATH}", normalize_host(&params.host));

    post_form(
        params.http.as_ref(),
        &url,
        &[
            ("grant_type", "refresh_token"),
            ("client_id", &params.client_id),
            ("client_secret", &params.client_secret),
            ("refresh_token", &params.refresh_token),
            ("redirect_uri", &params.redirect_uri),
        ],
    )
    .await
}

/// Kept next to its sibling so the two token calls read together; the Cloud one is JSON, this one is a form.
pub(crate) async fn refresh_cloud_token(
    http: Option<&reqwest::Client>,
    token_url: &str,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<TokenResponse> {
    let body = serde_json::json!({
        "grant_type": "refresh_token",
        "client_id": client_id,
        "client_secret": client_secret,
        "refresh_token": refresh_token,
    });

    post_json(http, token_url, &body).await
}

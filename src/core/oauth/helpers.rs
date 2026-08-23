use serde_json::{Value, json};

use crate::core::error::{Error, OAuthErrorDetails, Result};
use crate::core::oauth::types::{AccessibleResource, CallbackParams, TokenResponse};

pub(crate) const AUTHORIZE_URL: &str = "https://auth.atlassian.com/authorize";
pub(crate) const TOKEN_URL: &str = "https://auth.atlassian.com/oauth/token";
pub(crate) const ACCESSIBLE_RESOURCES_URL: &str = "https://api.atlassian.com/oauth/token/accessible-resources";
const DEFAULT_AUDIENCE: &str = "api.atlassian.com";

/// Where to send the user so they can grant access.
#[derive(Debug, Clone)]
pub struct AuthorizationUrlParams {
    pub client_id: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    /// Yours to generate and to verify when the callback comes back — it is what stops CSRF on the redirect.
    pub state: String,
    /// Defaults to `consent`.
    pub prompt: Option<String>,
    /// Defaults to `api.atlassian.com`.
    pub audience: Option<String>,
}

impl AuthorizationUrlParams {
    pub fn new(
        client_id: impl Into<String>,
        scopes: impl IntoIterator<Item = impl Into<String>>,
        redirect_uri: impl Into<String>,
        state: impl Into<String>,
    ) -> Self {
        AuthorizationUrlParams {
            client_id: client_id.into(),
            scopes: scopes.into_iter().map(Into::into).collect(),
            redirect_uri: redirect_uri.into(),
            state: state.into(),
            prompt: None,
            audience: None,
        }
    }
}

/// The authorization code to trade in, and the credentials to trade it with.
#[derive(Debug, Clone)]
pub struct ExchangeCodeParams {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    pub redirect_uri: String,
    /// The HTTP client to reach Atlassian by, so a proxy or a timeout covers the token calls too.
    pub http: Option<reqwest::Client>,
}

impl ExchangeCodeParams {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        code: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        ExchangeCodeParams {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            code: code.into(),
            redirect_uri: redirect_uri.into(),
            http: None,
        }
    }
}

/// The refresh credential set.
#[derive(Debug, Clone)]
pub struct RefreshTokenParams {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub http: Option<reqwest::Client>,
}

impl RefreshTokenParams {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        refresh_token: impl Into<String>,
    ) -> Self {
        RefreshTokenParams {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            refresh_token: refresh_token.into(),
            http: None,
        }
    }
}

/// Build the URL to send the user to so they can grant access.
pub fn generate_authorization_url(params: &AuthorizationUrlParams) -> String {
    let query = form_urlencoded::Serializer::new(String::new())
        .append_pair("audience", params.audience.as_deref().unwrap_or(DEFAULT_AUDIENCE))
        .append_pair("client_id", &params.client_id)
        .append_pair("scope", &params.scopes.join(" "))
        .append_pair("redirect_uri", &params.redirect_uri)
        .append_pair("state", &params.state)
        .append_pair("response_type", "code")
        .append_pair("prompt", params.prompt.as_deref().unwrap_or("consent"))
        .finish();

    format!("{AUTHORIZE_URL}?{query}")
}

/// Exchange the authorization `code` from the redirect callback for tokens.
pub async fn exchange_authorization_code(params: &ExchangeCodeParams) -> Result<TokenResponse> {
    let body = json!({
        "grant_type": "authorization_code",
        "client_id": params.client_id,
        "client_secret": params.client_secret,
        "code": params.code,
        "redirect_uri": params.redirect_uri,
    });

    post_json(params.http.as_ref(), TOKEN_URL, &body).await
}

/// Refresh an access token.
///
/// Atlassian rotates the refresh token on every call: persist the one that comes back and drop the old one, or the
/// next refresh fails.
pub async fn refresh_oauth2_token(params: &RefreshTokenParams) -> Result<TokenResponse> {
    let body = json!({
        "grant_type": "refresh_token",
        "client_id": params.client_id,
        "client_secret": params.client_secret,
        "refresh_token": params.refresh_token,
    });

    post_json(params.http.as_ref(), TOKEN_URL, &body).await
}

/// List the sites this access token can reach. The `id` of an entry is its cloud id.
pub async fn get_accessible_resources(
    access_token: &str,
    http: Option<&reqwest::Client>,
) -> Result<Vec<AccessibleResource>> {
    get_accessible_resources_at(ACCESSIBLE_RESOURCES_URL, access_token, http).await
}

/// The same lookup against a named URL. The endpoint is Atlassian's and fixed; naming it is what lets the manager's
/// own tests answer for it.
pub(crate) async fn get_accessible_resources_at(
    url: &str,
    access_token: &str,
    http: Option<&reqwest::Client>,
) -> Result<Vec<AccessibleResource>> {
    let owned;
    let http = match http {
        Some(client) => client,
        None => {
            owned = reqwest::Client::new();
            &owned
        }
    };

    let response = http
        .get(url)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {access_token}"))
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|error| oauth_transport_error(url, &error))?;

    read_json(url, response).await
}

/// Read the authorization code out of the URL Atlassian redirected the user to.
///
/// Handles the three ways this step goes wrong, each of which is easy to forget by hand: the user declined on the
/// consent screen, `state` is missing or does not match the one issued, or the URL is simply not a callback.
pub fn parse_callback_url(url: &str, expected_state: &str) -> Result<CallbackParams> {
    let parsed = url::Url::parse(url)
        .or_else(|_| url::Url::parse(&format!("http://localhost{url}")))
        .map_err(|error| Error::config(format!("The callback URL could not be parsed: {error}")))?;

    let mut code = None;
    let mut state = None;
    let mut error_code = None;
    let mut error_description = None;

    for (key, value) in parsed.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.into_owned()),
            "state" => state = Some(value.into_owned()),
            "error" => error_code = Some(value.into_owned()),
            "error_description" => error_description = Some(value.into_owned()),
            _ => {}
        }
    }

    if let Some(error_code) = error_code {
        let declined = error_code == "access_denied";
        let message = if declined {
            match &error_description {
                Some(description) => format!("The user declined authorization: {description}"),
                None => "The user declined authorization.".to_owned(),
            }
        } else {
            match &error_description {
                Some(description) => format!("Authorization failed: {error_code} — {description}"),
                None => format!("Authorization failed: {error_code}"),
            }
        };

        return Err(Error::oauth(
            message,
            OAuthErrorDetails {
                error: Some(error_code),
                error_description,
                reauthorization_required: declined,
                ..OAuthErrorDetails::default()
            },
        ));
    }

    let state = state.filter(|state| timing_safe_equals(state, expected_state)).ok_or_else(|| {
        Error::oauth(
            "The `state` in the callback does not match the one issued for this authorization request. The callback \
was not initiated by this session — discard it.",
            OAuthErrorDetails { error: Some("invalid_state".to_owned()), ..OAuthErrorDetails::default() },
        )
    })?;

    let code = code.filter(|code| !code.is_empty()).ok_or_else(|| {
        Error::oauth(
            "The callback URL carries neither an authorization code nor an error.",
            OAuthErrorDetails { error: Some("invalid_request".to_owned()), ..OAuthErrorDetails::default() },
        )
    })?;

    Ok(CallbackParams { code, state })
}

/// Constant-time string comparison, so a mismatch leaks nothing through timing.
fn timing_safe_equals(left: &str, right: &str) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut difference = 0u8;

    for (left, right) in left.as_bytes().iter().zip(right.as_bytes()) {
        difference |= left ^ right;
    }

    difference == 0
}

pub(crate) async fn post_json(http: Option<&reqwest::Client>, url: &str, body: &Value) -> Result<TokenResponse> {
    let owned;
    let http = match http {
        Some(client) => client,
        None => {
            owned = reqwest::Client::new();
            &owned
        }
    };

    let response = http
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .body(body.to_string())
        .send()
        .await
        .map_err(|error| oauth_transport_error(url, &error))?;

    read_json(url, response).await
}

pub(crate) async fn post_form(
    http: Option<&reqwest::Client>,
    url: &str,
    form: &[(&str, &str)],
) -> Result<TokenResponse> {
    let owned;
    let http = match http {
        Some(client) => client,
        None => {
            owned = reqwest::Client::new();
            &owned
        }
    };

    // Encoded into a `String` before the request is sent: the serializer holds a non-`Send` encoding callback, and
    // keeping it alive across the await would make every caller's future non-`Send` — no `tokio::spawn`, no
    // concurrent requests.
    let body = {
        let mut serializer = form_urlencoded::Serializer::new(String::new());

        for (key, value) in form {
            serializer.append_pair(key, value);
        }

        serializer.finish()
    };

    let response = http
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(reqwest::header::ACCEPT, "application/json")
        .body(body)
        .send()
        .await
        .map_err(|error| oauth_transport_error(url, &error))?;

    read_json(url, response).await
}

fn oauth_transport_error(url: &str, error: &reqwest::Error) -> Error {
    Error::oauth(format!("Request to {url} failed before a response arrived: {error}"), OAuthErrorDetails::default())
}

/// The transport does not fail on a non-2xx status, so the status is checked here and turned into an OAuth error.
async fn read_json<T: serde::de::DeserializeOwned>(url: &str, response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    let body: Value = serde_json::from_str(&text).unwrap_or_else(|_| Value::String(text.clone()));

    if !status.is_success() {
        let error = body.get("error").and_then(Value::as_str).map(ToOwned::to_owned);
        let error_description = body.get("error_description").and_then(Value::as_str).map(ToOwned::to_owned);
        let reauthorization_required = matches!(error.as_deref(), Some("invalid_grant" | "unauthorized_client"));
        let suffix = if text.is_empty() { String::new() } else { format!(": {text}") };

        return Err(Error::oauth(
            format!(
                "Request to {url} failed with status {} {}{suffix}",
                status.as_u16(),
                status.canonical_reason().unwrap_or_default()
            ),
            OAuthErrorDetails {
                status: Some(status.as_u16()),
                body: Some(body),
                error,
                error_description,
                reauthorization_required,
            },
        ));
    }

    serde_json::from_value(body).map_err(|error| {
        Error::oauth(
            format!("The answer from {url} was not the shape an OAuth 2.0 response has: {error}"),
            OAuthErrorDetails { status: Some(status.as_u16()), ..OAuthErrorDetails::default() },
        )
    })
}

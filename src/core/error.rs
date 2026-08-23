use std::fmt;
use std::time::{Duration, SystemTime};

use serde_json::Value;

use crate::core::product::SCOPE_HINT;

pub type Result<T> = std::result::Result<T, Error>;

/// Node/undici error codes that signal a recoverable transport-layer failure.
const TRANSIENT_NETWORK_MARKERS: &[&str] = &[
    "ECONNRESET",
    "ECONNREFUSED",
    "ETIMEDOUT",
    "ENOTFOUND",
    "EAI_AGAIN",
    "EPIPE",
];

/// HTTP statuses that signal a recoverable upstream failure. The single source of truth for both retry paths.
pub const TRANSIENT_HTTP_STATUSES: &[u16] = &[502, 503, 504];

pub fn is_transient_status(status: u16) -> bool {
    TRANSIENT_HTTP_STATUSES.contains(&status)
}

/// Which kind of API failure a non-2xx response describes, so callers branch on a name rather than a number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApiErrorKind {
    /// 401 — credentials missing, expired or rejected.
    Auth,
    /// 401 with `Unauthorized; scope does not match` — refreshing cannot help.
    Scope,
    /// 403 — authenticated, but not allowed.
    Forbidden,
    /// 404 — no such thing, or no permission to know it exists.
    NotFound,
    /// 429 — rate limited.
    RateLimit,
    /// 5xx — the API failed on its own side.
    Server,
    /// Any other non-2xx status.
    Other,
}

/// Everything a non-2xx response carried.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ApiErrorDetails {
    pub kind: ApiErrorKind,
    pub status: u16,
    pub status_text: String,
    /// Atlassian's error payload, parsed when it was JSON and a `Value::String` when it was not.
    pub body: Value,
    /// How long to wait before retrying, when the API sent `Retry-After`.
    pub retry_after: Option<Duration>,
}

/// Everything an OAuth 2.0 failure carried.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct OAuthErrorDetails {
    pub status: Option<u16>,
    pub body: Option<Value>,
    /// The OAuth 2.0 error code, e.g. `invalid_grant`. Branch on this rather than on `status`.
    pub error: Option<String>,
    pub error_description: Option<String>,
    /// Whether the grant is unrecoverable and the user has to authorize again.
    pub reauthorization_required: bool,
}

/// One place where the response and the schema disagreed.
#[derive(Debug, Clone)]
pub struct SchemaMismatchIssue {
    /// Dotted path to the value, e.g. `values.0.created`. Empty for the response root.
    pub path: String,
    /// What the schema expected there.
    pub expected: String,
    /// What arrived, named by its type rather than quoted.
    pub received: String,
}

/// What a caller is told when a response does not match its schema.
///
/// Types and paths, near enough: this is meant to be pasted into a bug report, and the body it describes belongs to
/// whoever ran the request.
#[derive(Debug, Clone)]
pub struct SchemaMismatchReport {
    /// Method and path, without the query string.
    pub endpoint: String,
    pub issues: Vec<SchemaMismatchIssue>,
}

impl fmt::Display for SchemaMismatchReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.endpoint)?;

        for issue in &self.issues {
            let where_ = if issue.path.is_empty() {
                "the response root".to_owned()
            } else {
                format!("`{}`", issue.path)
            };

            write!(
                f,
                "\n  at {}, expected {}, got {}",
                where_, issue.expected, issue.received
            )?;
        }

        Ok(())
    }
}

/// Every failure this crate produces.
///
/// Branch on the predicates — [`Error::is_not_found`], [`Error::is_rate_limit`] and the rest — rather than on the
/// variant where you only care about one condition: they read the status and the OAuth code for you.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API returned a non-2xx HTTP response.
    #[error("{message}")]
    Api {
        message: String,
        details: Box<ApiErrorDetails>,
    },

    /// The request never produced an HTTP response — DNS, TLS, a reset socket, a timeout, an unreachable host.
    #[error("{message}")]
    Network {
        message: String,
        /// Whether the failure looks retryable rather than permanent.
        transient: bool,
        #[source]
        source: reqwest::Error,
    },

    /// An OAuth 2.0 failure: token exchange, refresh, or cloud-id resolution.
    ///
    /// Deliberately not an [`Error::Api`]: it does not come from the product API, and a caller retrying product calls
    /// should not treat "your refresh token is dead" as the same class of problem as "that page is missing".
    #[error("{message}")]
    OAuth {
        message: String,
        details: Box<OAuthErrorDetails>,
    },

    /// The client was configured in a way that cannot work — contradictory options, or a required one missing.
    #[error("{0}")]
    Config(String),

    /// A request body could not be written as JSON.
    ///
    /// A generated model cannot produce this — it has no custom serialization to fail in — so in practice it means a
    /// `serde_json::Value` handed to an operation held something JSON has no way to write, such as a map keyed by
    /// anything but a string.
    #[error("A request body could not be serialized as JSON: {source}")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },

    /// The request succeeded, but the response is not what the endpoint's schema describes.
    ///
    /// Covers both ways that can happen: the response was not JSON at all, or it parsed as JSON and its shape had
    /// drifted.
    #[error("Response did not match the schema for {}", .report.endpoint)]
    SchemaMismatch {
        report: Box<SchemaMismatchReport>,
        #[source]
        source: Option<serde_json::Error>,
    },
}

impl Error {
    pub fn config(message: impl Into<String>) -> Self {
        Error::Config(message.into())
    }

    pub fn oauth(message: impl Into<String>, details: OAuthErrorDetails) -> Self {
        Error::OAuth {
            message: message.into(),
            details: Box::new(details),
        }
    }

    /// Any non-2xx response from the API, whatever the status.
    pub fn is_api(&self) -> bool {
        matches!(self, Error::Api { .. })
    }

    fn api_kind(&self) -> Option<ApiErrorKind> {
        match self {
            Error::Api { details, .. } => Some(details.kind),
            _ => None,
        }
    }

    /// 401 — credentials missing, expired or rejected. True for a scope error too, which is still a 401.
    pub fn is_auth(&self) -> bool {
        matches!(self.api_kind(), Some(ApiErrorKind::Auth | ApiErrorKind::Scope))
    }

    /// The token is valid but lacks a scope this endpoint requires.
    ///
    /// Distinct from [`Error::is_auth`] because the remedy is different: refreshing cannot help, the app needs the
    /// scope added and the user has to consent again.
    pub fn is_scope(&self) -> bool {
        self.api_kind() == Some(ApiErrorKind::Scope)
    }

    /// 403 — authenticated but not permitted.
    pub fn is_forbidden(&self) -> bool {
        self.api_kind() == Some(ApiErrorKind::Forbidden)
    }

    /// 404 — absent, or invisible to you.
    pub fn is_not_found(&self) -> bool {
        self.api_kind() == Some(ApiErrorKind::NotFound)
    }

    /// 429 — rate limited. Read [`Error::retry_after`] for Atlassian's own advice.
    pub fn is_rate_limit(&self) -> bool {
        self.api_kind() == Some(ApiErrorKind::RateLimit)
    }

    /// 5xx — the API failed on its side.
    pub fn is_server(&self) -> bool {
        self.api_kind() == Some(ApiErrorKind::Server)
    }

    /// No HTTP response at all — DNS, TLS, socket, timeout.
    pub fn is_network(&self) -> bool {
        matches!(self, Error::Network { .. })
    }

    pub fn is_oauth(&self) -> bool {
        matches!(self, Error::OAuth { .. })
    }

    pub fn is_config(&self) -> bool {
        matches!(self, Error::Config(_))
    }

    pub fn is_schema_mismatch(&self) -> bool {
        matches!(self, Error::SchemaMismatch { .. })
    }

    /// A request body could not be written as JSON.
    pub fn is_serialization(&self) -> bool {
        matches!(self, Error::Serialization { .. })
    }

    /// The grant is gone: no refresh will fix it, and the user has to authorize again.
    ///
    /// Deliberately narrower than "any OAuth failure". A wrong client secret also fails the token endpoint — with
    /// `access_denied`, the same code a declining user produces — and sending people through consent over a bad
    /// environment variable would loop them forever.
    pub fn is_reauthorization_required(&self) -> bool {
        match self {
            Error::OAuth { details, .. } => details.reauthorization_required,
            _ => false,
        }
    }

    /// Whether retrying stands a chance: a transient transport failure or a 502/503/504.
    pub fn is_transient(&self) -> bool {
        match self {
            Error::Network { transient, .. } => *transient,
            Error::Api { details, .. } => is_transient_status(details.status),
            _ => false,
        }
    }

    /// The HTTP status, for the failures that carry one.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Api { details, .. } => Some(details.status),
            Error::OAuth { details, .. } => details.status,
            _ => None,
        }
    }

    /// Atlassian's error payload, for the failures that carry one.
    pub fn body(&self) -> Option<&Value> {
        match self {
            Error::Api { details, .. } => Some(&details.body),
            Error::OAuth { details, .. } => details.body.as_ref(),
            _ => None,
        }
    }

    /// How long the API asked you to wait, from `Retry-After`.
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Error::Api { details, .. } => details.retry_after,
            _ => None,
        }
    }

    /// The OAuth 2.0 error code from the auth server, e.g. `invalid_grant`.
    pub fn oauth_code(&self) -> Option<&str> {
        match self {
            Error::OAuth { details, .. } => details.error.as_deref(),
            _ => None,
        }
    }

    /// Which fields disagreed with the schema, by path and type.
    pub fn schema_report(&self) -> Option<&SchemaMismatchReport> {
        match self {
            Error::SchemaMismatch { report, .. } => Some(report),
            _ => None,
        }
    }
}

/// Whether a transport failure is worth retrying.
///
/// `reqwest` does not expose the operating system's error code, so the chain is rendered and matched instead. Broken
/// TLS sessions count, as do the connect and read timeouts the client itself imposes.
pub fn is_transient_transport_failure(err: &reqwest::Error) -> bool {
    if err.is_timeout() || err.is_connect() {
        return true;
    }

    if err.is_body() {
        return true;
    }

    let mut rendered = err.to_string();
    let mut source: Option<&(dyn std::error::Error + 'static)> = std::error::Error::source(err);

    for _ in 0..5 {
        let Some(current) = source else { break };

        rendered.push(' ');
        rendered.push_str(&current.to_string());
        source = current.source();
    }

    let upper = rendered.to_uppercase();

    TRANSIENT_NETWORK_MARKERS.iter().any(|marker| upper.contains(marker))
        || upper.contains("CONNECTION RESET")
        || upper.contains("CONNECTION REFUSED")
        || upper.contains("BROKEN PIPE")
        || upper.contains("ERR_SSL")
}

/// Wrap whatever the transport rejected with into [`Error::Network`], preserving the original as the source.
pub fn to_network_error(err: reqwest::Error, url: &str) -> Error {
    let transient = is_transient_transport_failure(&err);

    Error::Network {
        message: format!("Request to {url} failed: {err}"),
        transient,
        source: err,
    }
}

/// `Retry-After` as a duration. The header is either delta-seconds or an HTTP date; both are accepted, and anything
/// else is ignored rather than guessed at.
pub fn parse_retry_after(header: Option<&str>, now: SystemTime) -> Option<Duration> {
    let header = header?.trim();

    if let Ok(seconds) = header.parse::<f64>()
        && seconds.is_finite()
    {
        return Some(Duration::from_secs_f64(seconds.max(0.0)));
    }

    let date = httpdate::parse_http_date(header).ok()?;

    Some(date.duration_since(now).unwrap_or(Duration::ZERO))
}

/// Whether a 401 is really a missing scope.
///
/// The API says so in the body — `{"code":401,"message":"Unauthorized; scope does not match"}` — and nowhere else.
/// Matched loosely, since the wording is Atlassian's to change; a miss only costs the caller a plain auth error.
fn is_scope_mismatch(body: &Value) -> bool {
    let message = match body {
        Value::Object(map) => map.get("message").and_then(Value::as_str),
        Value::String(text) => Some(text.as_str()),
        _ => None,
    };

    message.is_some_and(|message| message.to_lowercase().contains("scope does not match"))
}

/// Build the error that matches the status, so callers can branch on a kind instead of a number.
pub fn create_api_error(
    message: String,
    status: u16,
    status_text: String,
    body: Value,
    retry_after: Option<Duration>,
) -> Error {
    let (kind, message) = match status {
        401 if is_scope_mismatch(&body) => (
            ApiErrorKind::Scope,
            format!(
                "{message}\n\nThe token is missing a scope this endpoint requires. Refreshing will not help: add \
the scope in the developer console and have the user authorize again. {SCOPE_HINT}"
            ),
        ),
        401 => (ApiErrorKind::Auth, message),
        403 => (ApiErrorKind::Forbidden, message),
        404 => (ApiErrorKind::NotFound, message),
        429 => (ApiErrorKind::RateLimit, message),
        status if status >= 500 => (ApiErrorKind::Server, message),
        _ => (ApiErrorKind::Other, message),
    };

    Error::Api {
        message,
        details: Box::new(ApiErrorDetails {
            kind,
            status,
            status_text,
            body,
            retry_after,
        }),
    }
}

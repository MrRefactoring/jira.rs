use std::collections::BTreeMap;
use std::fmt;
use std::time::{Duration, SystemTime};

use serde_json::Value;

use crate::core::product::SCOPE_HINT;

/// The result of anything this crate does.
pub type Result<T> = std::result::Result<T, Error>;

/// Node/undici error codes that signal a recoverable transport-layer failure.
const TRANSIENT_NETWORK_MARKERS: &[&str] =
    &["ECONNRESET", "ECONNREFUSED", "ETIMEDOUT", "ENOTFOUND", "EAI_AGAIN", "EPIPE"];

/// HTTP statuses that signal a recoverable upstream failure. The single source of truth for both retry paths.
pub const TRANSIENT_HTTP_STATUSES: &[u16] = &[502, 503, 504];

/// Whether a status is one of the gateway failures worth retrying.
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
    /// What kind of failure the status describes.
    pub kind: ApiErrorKind,
    /// The HTTP status that crossed the wire.
    pub status: u16,
    /// The status's reason phrase, e.g. `Not Found`.
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
    /// The HTTP status of the token call, when one was answered.
    pub status: Option<u16>,
    /// What the authorization server said, parsed when it was JSON.
    pub body: Option<Value>,
    /// The OAuth 2.0 error code, e.g. `invalid_grant`. Branch on this rather than on `status`.
    pub error: Option<String>,
    /// The authorization server's prose about the error.
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
    /// Every place the two disagreed.
    pub issues: Vec<SchemaMismatchIssue>,
}

impl fmt::Display for SchemaMismatchReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.endpoint)?;

        for issue in &self.issues {
            let where_ =
                if issue.path.is_empty() { "the response root".to_owned() } else { format!("`{}`", issue.path) };

            write!(f, "\n  at {}, expected {}, got {}", where_, issue.expected, issue.received)?;
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
        /// The status, and what the body said.
        message: String,
        /// Everything the response carried.
        details: Box<ApiErrorDetails>,
    },

    /// The request never produced an HTTP response — DNS, TLS, a reset socket, a timeout, an unreachable host.
    #[error("{message}")]
    Network {
        /// The URL that was being reached, and what went wrong.
        message: String,
        /// Whether the failure looks retryable rather than permanent.
        transient: bool,
        /// What the transport reported.
        #[source]
        source: reqwest::Error,
    },

    /// An OAuth 2.0 failure: token exchange, refresh, or cloud-id resolution.
    ///
    /// Deliberately not an [`Error::Api`]: it does not come from the product API, and a caller retrying product calls
    /// should not treat "your refresh token is dead" as the same class of problem as "that page is missing".
    #[error("{message}")]
    OAuth {
        /// What was being done, and what the authorization server said.
        message: String,
        /// Everything the token call carried.
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
        /// What the serializer reported.
        #[from]
        source: serde_json::Error,
    },

    /// The request succeeded, but the response is not what the endpoint's schema describes.
    ///
    /// Covers both ways that can happen: the response was not JSON at all, or it parsed as JSON and its shape had
    /// drifted.
    #[error("Response did not match the schema for {}", .report.endpoint)]
    SchemaMismatch {
        /// Where the response and the schema disagreed, by path and type.
        report: Box<SchemaMismatchReport>,
        /// The deserializer's own complaint, when there was one.
        #[source]
        source: Option<serde_json::Error>,
    },
}

impl Error {
    /// A configuration that cannot work, described.
    pub fn config(message: impl Into<String>) -> Self {
        Error::Config(message.into())
    }

    /// An OAuth 2.0 failure, described.
    pub fn oauth(message: impl Into<String>, details: OAuthErrorDetails) -> Self {
        Error::OAuth { message: message.into(), details: Box::new(details) }
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

    /// The token endpoint or the cloud-id lookup failed. Not a product API failure.
    pub fn is_oauth(&self) -> bool {
        matches!(self, Error::OAuth { .. })
    }

    /// The client was configured in a way that cannot work.
    pub fn is_config(&self) -> bool {
        matches!(self, Error::Config(_))
    }

    /// The response arrived but is not what the schema describes. Read [`Error::schema_report`] for where.
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

    /// What Jira said went wrong with the request as a whole, from `errorMessages`.
    ///
    /// Jira Service Management reports through a single `errorMessage` instead, and that is read too, so one call
    /// covers both products. Empty for anything but an API failure, and for a body that carried neither.
    pub fn error_messages(&self) -> Vec<&str> {
        let Some(Value::Object(body)) = self.body() else { return Vec::new() };
        let listed =
            body.get("errorMessages").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str);
        let single = body.get("errorMessage").and_then(Value::as_str);

        listed.chain(single).collect()
    }

    /// What Jira said went wrong with each field, from `errors`: the field's name against the complaint about it.
    ///
    /// A rejected issue creation lands here rather than in [`Error::error_messages`] — `summary` against
    /// `You must specify a summary of the issue`, say. Sorted by field, and empty for anything but an API failure.
    pub fn field_errors(&self) -> BTreeMap<&str, &str> {
        let Some(Value::Object(body)) = self.body() else { return BTreeMap::new() };

        body.get("errors")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
            .filter_map(|(field, message)| Some((field.as_str(), message.as_str()?)))
            .collect()
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

    Error::Network { message: format!("Request to {url} failed: {err}"), transient, source: err }
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

    Error::Api { message, details: Box::new(ApiErrorDetails { kind, status, status_text, body, retry_after }) }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn api_error(status: u16, body: serde_json::Value) -> Error {
        create_api_error(format!("Request failed: {status}"), status, "Error".to_owned(), body, None)
    }

    #[test]
    fn maps_each_status_to_the_kind_that_names_it() {
        let cases = [
            (401, ApiErrorKind::Auth),
            (403, ApiErrorKind::Forbidden),
            (404, ApiErrorKind::NotFound),
            (429, ApiErrorKind::RateLimit),
            (500, ApiErrorKind::Server),
            (503, ApiErrorKind::Server),
        ];

        for (status, expected) in cases {
            let error = api_error(status, json!({}));

            match error {
                Error::Api { details, .. } => assert_eq!(details.kind, expected, "status {status}"),
                other => panic!("expected an API error for {status}, got {other:?}"),
            }
        }
    }

    #[test]
    fn leaves_an_unremarkable_4xx_as_a_plain_api_error() {
        let error = api_error(418, json!({}));

        assert!(error.is_api());
        assert!(!error.is_auth());
        assert!(!error.is_not_found());
        assert_eq!(error.status(), Some(418));
    }

    #[test]
    fn keeps_every_kind_catchable_as_an_api_error() {
        for status in [401, 403, 404, 429, 500] {
            assert!(api_error(status, json!({})).is_api(), "status {status}");
        }
    }

    #[test]
    fn carries_the_parsed_body() {
        let error = api_error(400, json!({ "errorMessages": ["Field 'foo' cannot be set"] }));

        assert_eq!(error.body().unwrap()["errorMessages"][0], "Field 'foo' cannot be set");
    }

    #[test]
    fn tells_the_request_level_messages_from_the_field_level_ones() {
        let error = api_error(
            400,
            json!({
                "errorMessages": ["Issue does not exist or you do not have permission to see it."],
                "errors": {
                    "summary": "You must specify a summary of the issue.",
                    "priority": "Priority is required."
                }
            }),
        );

        assert_eq!(error.error_messages(), vec!["Issue does not exist or you do not have permission to see it."]);
        assert_eq!(
            error.field_errors().into_iter().collect::<Vec<_>>(),
            vec![("priority", "Priority is required."), ("summary", "You must specify a summary of the issue.")]
        );
    }

    #[test]
    fn reads_service_managements_single_message_as_a_message_too() {
        let error = api_error(
            404,
            json!({ "errorMessage": "Request type not found.", "i18nErrorMessage": { "i18nKey": "sd.request.type.not.found" } }),
        );

        assert_eq!(error.error_messages(), vec!["Request type not found."]);
        assert!(error.field_errors().is_empty());
    }

    #[test]
    fn has_no_messages_to_offer_when_the_body_is_not_jiras_shape() {
        assert!(api_error(502, json!("Bad Gateway")).error_messages().is_empty());
        assert!(api_error(400, json!({ "errors": { "summary": 42 } })).field_errors().is_empty());
        assert!(Error::config("`host` is required").error_messages().is_empty());
        assert!(Error::config("`host` is required").field_errors().is_empty());
    }

    #[test]
    fn classifies_the_scope_401_as_its_own_kind() {
        let error = api_error(401, json!({ "code": 401, "message": "Unauthorized; scope does not match" }));

        assert!(error.is_scope());
        // Still a 401, so anything catching an auth failure keeps catching this one.
        assert!(error.is_auth());
    }

    #[test]
    fn names_what_to_do_about_a_missing_scope_in_the_message() {
        let error = api_error(401, json!({ "message": "Unauthorized; scope does not match" }));
        let message = error.to_string();

        assert!(message.contains("Refreshing will not help"), "{message}");
        assert!(message.contains("developer console"), "{message}");
    }

    #[test]
    fn does_not_mistake_an_ordinary_401_for_a_scope_failure() {
        let error = api_error(401, json!({ "message": "Client must be authenticated" }));

        assert!(error.is_auth());
        assert!(!error.is_scope());
    }

    #[test]
    fn reads_retry_after_given_in_seconds() {
        assert_eq!(parse_retry_after(Some("30"), SystemTime::now()), Some(Duration::from_secs(30)));
    }

    #[test]
    fn reads_retry_after_given_as_an_http_date() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let later = httpdate::fmt_http_date(now + Duration::from_secs(120));

        let parsed = parse_retry_after(Some(&later), now).expect("an HTTP date is a Retry-After");

        assert_eq!(parsed.as_secs(), 120);
    }

    #[test]
    fn leaves_retry_after_absent_when_the_header_is_missing_or_nonsense() {
        assert_eq!(parse_retry_after(None, SystemTime::now()), None);
        assert_eq!(parse_retry_after(Some("soon"), SystemTime::now()), None);
    }

    #[test]
    fn reports_a_retry_after_in_the_past_as_no_wait_at_all() {
        let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
        let earlier = httpdate::fmt_http_date(now - Duration::from_secs(120));

        assert_eq!(parse_retry_after(Some(&earlier), now), Some(Duration::ZERO));
    }

    #[test]
    fn a_transient_status_is_worth_retrying_and_a_404_is_not() {
        assert!(api_error(503, json!({})).is_transient());
        assert!(api_error(502, json!({})).is_transient());
        assert!(!api_error(404, json!({})).is_transient());
        assert!(!api_error(429, json!({})).is_transient());
        assert!(!api_error(500, json!({})).is_transient());
    }

    #[test]
    fn the_predicates_do_not_confuse_siblings() {
        let not_found = api_error(404, json!({}));

        assert!(not_found.is_not_found());
        assert!(!not_found.is_forbidden());
        assert!(!not_found.is_network());
        assert!(!not_found.is_oauth());
        assert!(!not_found.is_schema_mismatch());
        assert!(!not_found.is_config());
    }

    #[test]
    fn a_config_failure_is_not_an_api_failure() {
        let error = Error::config("`host` is required");

        assert!(error.is_config());
        assert!(!error.is_api());
        assert_eq!(error.status(), None);
        assert_eq!(error.to_string(), "`host` is required");
    }
}

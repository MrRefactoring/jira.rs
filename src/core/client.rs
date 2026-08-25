use std::sync::Arc;
use std::time::{Duration, SystemTime};

use bytes::Bytes;
use reqwest::Method;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::core::auth::{Auth, BoxFuture};
use crate::core::body::{Body, FORM_URLENCODED, json_to_form, should_set_json_content_type};
use crate::core::error::{
    ApiErrorDetails, ApiErrorKind, Error, Result, SchemaMismatchIssue, SchemaMismatchReport, create_api_error,
    is_transient_status, parse_retry_after, to_network_error,
};
use crate::core::oauth::OAuth2Manager;
use crate::core::product::USER_AGENT;
use crate::core::query::{QueryValue, build_url_with_search_params};
use crate::core::retry::RetryConfig;

/// Supplies fresh credentials once, when the ones in hand are refused.
pub trait AuthRefresher: Send + Sync + 'static {
    fn refresh(&self) -> BoxFuture<'_, Result<Auth>>;
}

impl<F, Fut> AuthRefresher for F
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<Auth>> + Send + 'static,
{
    fn refresh(&self) -> BoxFuture<'_, Result<Auth>> {
        Box::pin(self())
    }
}

/// One request, in the shape the transport sends it.
///
/// Generated operations build this and hand it to [`Client::send`]; [`Client::request`] is the same thing by hand.
#[derive(Debug, Clone, Default)]
pub struct RequestConfig {
    pub method: Method,
    /// The path, e.g. `/rest/api/3/myself`. Joined onto the client's host.
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, QueryValue)>,
    pub body: Option<Body>,
    /// The media type to send the body as, for the endpoints that do not take JSON.
    pub content_type: Option<String>,
}

impl RequestConfig {
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        RequestConfig { method, url: url.into(), ..RequestConfig::default() }
    }

    /// Method and path, without the query string — what an error names the endpoint by.
    fn endpoint(&self) -> String {
        format!("{} {}", self.method, self.url)
    }
}

/// The `X-Seraph-LoginReason` values that mean the credentials were presented and refused.
///
/// `AUTHORISATION_FAILED` is deliberately absent: it means the user is who they claim and merely lacks a permission,
/// which the status already says. `OUT` is absent for the same kind of reason — a Data Center instance behind SSO
/// sets it on responses that are perfectly legitimate.
const SERAPH_LOGIN_FAILURES: &[&str] = &["AUTHENTICATED_FAILED", "AUTHENTICATION_DENIED"];

struct Inner {
    http: reqwest::Client,
    host: Option<String>,
    auth: Option<Auth>,
    headers: Vec<(String, String)>,
    retry: RetryConfig,
    oauth: Option<OAuth2Manager>,
    get_auth_on_401: Option<Arc<dyn AuthRefresher>>,
}

/// A low-level API client.
///
/// It carries only transport, auth and retry policy — it knows nothing about any one API surface, so a single
/// instance drives every surface this crate exposes, and with it a single set of credentials. Cloning is cheap: every
/// clone shares one connection pool and one OAuth token state.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Client")
            .field("host", &self.inner.host)
            .field("auth", &self.inner.auth)
            .finish_non_exhaustive()
    }
}

impl Client {
    /// Starts building a client. `host` is the bare site URL — the API path belongs to the request, not here.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// The site this client sends to, as configured.
    ///
    /// Absent under Cloud OAuth 2.0 (3LO), where the base URL is derived per request from the accessible resources.
    pub fn host(&self) -> Option<&str> {
        self.inner.host.as_deref()
    }

    pub fn request(&self, method: Method, url: impl Into<String>) -> RequestBuilder {
        RequestBuilder { client: self.clone(), config: RequestConfig::new(method, url) }
    }

    pub fn get(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    pub fn post(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    pub fn put(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::PUT, url)
    }

    pub fn patch(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    pub fn delete(&self, url: impl Into<String>) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }

    /// Sends the request and deserializes the JSON body into `T`.
    pub async fn send<T: DeserializeOwned>(&self, config: &RequestConfig) -> Result<T> {
        let response = self.execute(config).await?;
        let endpoint = config.endpoint();

        if response.body.is_empty() {
            return deserialize_at(&endpoint, &Value::Null);
        }

        if !response.is_json() {
            return Err(Error::SchemaMismatch {
                report: Box::new(SchemaMismatchReport {
                    endpoint,
                    issues: vec![SchemaMismatchIssue {
                        path: String::new(),
                        expected: "application/json".to_owned(),
                        received: response.content_type().unwrap_or("no content type").to_owned(),
                    }],
                }),
                source: None,
            });
        }

        // A body the API labelled as JSON and did not send as JSON falls back to text rather than failing here:
        // Jira mislabels a handful of plain-text responses, and an endpoint typed as a string still reads them.
        let value: Value = serde_json::from_slice(&response.body).unwrap_or_else(|_| Value::String(response.text()));

        #[cfg(feature = "audit")]
        record_undocumented_keys::<T>(&endpoint, &value);

        deserialize_at(&endpoint, &value)
    }

    /// Sends the request and hands back the JSON body unmodelled.
    pub async fn send_raw(&self, config: &RequestConfig) -> Result<Value> {
        let response = self.execute(config).await?;

        if response.body.is_empty() {
            return Ok(Value::Null);
        }

        Ok(serde_json::from_slice(&response.body)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&response.body).into_owned())))
    }

    /// Sends the request and hands back the raw bytes, for the endpoints that answer with a file.
    pub async fn send_bytes(&self, config: &RequestConfig) -> Result<Bytes> {
        Ok(self.execute(config).await?.body)
    }

    /// Sends the request and discards the body, for the endpoints that answer with nothing worth reading.
    pub async fn send_empty(&self, config: &RequestConfig) -> Result<()> {
        self.execute(config).await.map(|_| ())
    }

    /// The whole request loop: auth, retry, re-authentication and the error the status maps to.
    async fn execute(&self, config: &RequestConfig) -> Result<RawResponse> {
        #[cfg(feature = "coverage")]
        crate::core::coverage::record(&config.endpoint());

        let base = match &self.inner.oauth {
            Some(manager) => Some(manager.base_url().await?),
            None => self.inner.host.clone(),
        };

        let absolute = config.url.starts_with("http://") || config.url.starts_with("https://");
        let path =
            if config.url.starts_with('/') || absolute { config.url.clone() } else { format!("/{}", config.url) };

        let url = match base {
            Some(base) if !absolute => format!("{}{path}", base.trim_end_matches('/')),
            _ => path,
        };
        let url = build_url_with_search_params(&url, &config.query);

        let mut credential = self.current_credential().await?;
        let mut attempt = 0;
        let mut delay = self.inner.retry.initial_delay;
        let mut reauthenticated = false;
        let max_attempts = self.inner.retry.max_attempts.max(1);

        let response = loop {
            match self.send_once(config, &url, credential.header.as_deref()).await {
                Err(error) => {
                    let network = to_network_error(error, &url);

                    if attempt + 1 < max_attempts && network.is_transient() {
                        attempt += 1;
                        tokio::time::sleep(delay).await;
                        delay = self.inner.retry.next_delay(delay);
                        continue;
                    }

                    return Err(network);
                }
                Ok(response) => {
                    let has_auth = self.inner.auth.is_some();
                    let unauthenticated = response.status == 401 || credentials_rejected(&response.headers, has_auth);

                    if unauthenticated && !reauthenticated && !is_scope_mismatch_body(&response.body) {
                        if let Some(manager) = &self.inner.oauth
                            && manager.can_refresh().await
                        {
                            reauthenticated = true;
                            manager.force_refresh(credential.generation).await?;
                            credential = self.current_credential().await?;
                            continue;
                        }

                        if let Some(refresher) = &self.inner.get_auth_on_401 {
                            reauthenticated = true;
                            credential = Credential {
                                header: refresher.refresh().await?.authorization_header().await?,
                                generation: credential.generation,
                            };
                            continue;
                        }
                    }

                    if is_transient_status(response.status) && attempt + 1 < max_attempts {
                        attempt += 1;
                        tokio::time::sleep(delay).await;
                        delay = self.inner.retry.next_delay(delay);
                        continue;
                    }

                    break response;
                }
            }
        };

        // Ahead of the status, because the whole point is a `200` that is really a rejection: left to the branch
        // below, an anonymous-scope body would be handed back as a successful result.
        if credentials_rejected(&response.headers, self.inner.auth.is_some()) {
            return Err(rejected_credentials_error(&response));
        }

        if !(200..300).contains(&response.status) {
            let text = response.text();
            let body = response.json_body();
            let suffix = if text.is_empty() { String::new() } else { format!(" - {text}") };

            return Err(create_api_error(
                format!("Request failed: {} {}{suffix}", response.status, response.status_text),
                response.status,
                response.status_text.clone(),
                body,
                parse_retry_after(response.header("retry-after"), SystemTime::now()),
            ));
        }

        Ok(response)
    }

    async fn send_once(
        &self,
        config: &RequestConfig,
        url: &str,
        auth_header: Option<&str>,
    ) -> std::result::Result<RawResponse, reqwest::Error> {
        let mut request = self.inner.http.request(config.method.clone(), url);
        let mut headers: Vec<(String, String)> = vec![("accept".to_owned(), "application/json".to_owned())];

        // A declared media type wins outright; a form body states its own, and a multipart body needs the transport
        // to set the boundary.
        match (&config.content_type, &config.body) {
            (Some(_), Some(Body::Form(_)) | Some(Body::Multipart(_))) => {}
            (Some(content_type), _) => headers.push(("content-type".to_owned(), content_type.clone())),
            (None, body) if should_set_json_content_type(body.as_ref(), &config.method) => {
                headers.push(("content-type".to_owned(), "application/json".to_owned()));
            }
            _ => {}
        }

        if let Some(header) = auth_header {
            headers.push(("authorization".to_owned(), header.to_owned()));
        }

        headers.extend(self.inner.headers.iter().cloned());
        headers.extend(config.headers.iter().cloned());

        let mut header_map = HeaderMap::new();

        for (name, value) in headers {
            if let (Ok(name), Ok(value)) = (HeaderName::from_bytes(name.as_bytes()), HeaderValue::from_str(&value)) {
                header_map.insert(name, value);
            }
        }

        request = request.headers(header_map);

        request = match &config.body {
            None => request,
            Some(Body::Json(value)) => {
                if config.content_type.as_deref() == Some(FORM_URLENCODED) {
                    request.form(&json_to_form(value))
                } else {
                    request.body(value.to_string())
                }
            }
            Some(Body::Text(text)) => request.body(text.clone()),
            Some(Body::Form(entries)) => request.form(entries),
            Some(Body::Bytes(bytes)) => request.body(bytes.clone()),
            Some(Body::Multipart(multipart)) => request.multipart(multipart.to_form()),
        };

        let response = request.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;

        Ok(RawResponse {
            status: status.as_u16(),
            status_text: status.canonical_reason().unwrap_or_default().to_owned(),
            headers,
            body,
        })
    }

    async fn current_credential(&self) -> Result<Credential> {
        if let Some(manager) = &self.inner.oauth {
            let (header, generation) = manager.authorization_header().await?;

            return Ok(Credential { header: Some(header), generation });
        }

        let header = match &self.inner.auth {
            Some(auth) => auth.authorization_header().await?,
            None => None,
        };

        Ok(Credential { header, generation: 0 })
    }
}

struct Credential {
    header: Option<String>,
    generation: u64,
}

struct RawResponse {
    status: u16,
    status_text: String,
    headers: HeaderMap,
    body: Bytes,
}

impl RawResponse {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|value| value.to_str().ok())
    }

    fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    fn is_json(&self) -> bool {
        self.content_type().is_some_and(|value| value.contains("application/json"))
    }

    fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    /// Atlassian's error payload, parsed when it was JSON and the raw text when it was not.
    fn json_body(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or_else(|_| Value::String(self.text()))
    }
}

/// Whether the credentials were refused, whatever status the response carries.
///
/// An endpoint that permits anonymous access answers `200` with an anonymous-scope body when the API token is expired
/// or wrong — an empty list where the caller expected their own data — and says so nowhere but this header.
fn credentials_rejected(headers: &HeaderMap, has_auth: bool) -> bool {
    if !has_auth {
        return false;
    }

    headers
        .get("x-seraph-loginreason")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|reason| SERAPH_LOGIN_FAILURES.contains(&reason))
}

fn rejected_credentials_error(response: &RawResponse) -> Error {
    let reason = response.header("x-seraph-loginreason").unwrap_or_default();
    // Data Center adds this after too many failed sign-ins and then refuses the right password too, naming the page a
    // human has to visit. Advice about an expired token would send that caller looking in the wrong place.
    let challenge = response.header("x-authentication-denied-reason");
    let advice = challenge.map_or_else(
        || "The API token or password may be expired, revoked or mistyped.".to_owned(),
        |challenge| {
            format!(
                "The credentials may well be correct: Jira is refusing the sign-in until a challenge is answered \
({challenge})."
            )
        },
    );
    let text = response.text();
    let anonymously = if (200..300).contains(&response.status) { " and answered as an anonymous user" } else { "" };
    let suffix = if text.is_empty() { String::new() } else { format!(" - {text}") };

    // The status is the one that crossed the wire, not 401: an endpoint permitting anonymous access reports the
    // refusal on a `200`, and recording 401 there would name a status that never happened. The kind stays `Auth`
    // regardless, because that is what went wrong.
    Error::Api {
        message: format!(
            "Request failed: Jira rejected the credentials (x-seraph-loginreason: {reason}){anonymously}. \
{advice}{suffix}"
        ),
        details: Box::new(ApiErrorDetails {
            kind: ApiErrorKind::Auth,
            status: response.status,
            status_text: response.status_text.clone(),
            body: response.json_body(),
            retry_after: None,
        }),
    }
}

/// Whether this 401 means "missing scope" rather than "stale token". Refreshing cannot fix the former.
fn is_scope_mismatch_body(body: &Bytes) -> bool {
    String::from_utf8_lossy(body).to_lowercase().contains("scope does not match")
}

fn deserialize_at<T: DeserializeOwned>(endpoint: &str, value: &Value) -> Result<T> {
    match serde_path_to_error::deserialize(value) {
        Ok(parsed) => Ok(parsed),
        Err(error) => {
            let path = error.path().to_string();
            let path = if path == "." { String::new() } else { path };
            let received = describe_value_at_path(value, &path);

            Err(Error::SchemaMismatch {
                report: Box::new(SchemaMismatchReport {
                    endpoint: endpoint.to_owned(),
                    issues: vec![SchemaMismatchIssue { path, expected: error.inner().to_string(), received }],
                }),
                source: None,
            })
        }
    }
}

/// The value at a dotted path, named by its type rather than quoted.
///
/// The report is meant to be pasted into a bug report, and the body it describes belongs to whoever ran the request —
/// issue summaries, account names, custom field contents. A report that leaks those turns a schema bug into someone
/// else's incident.
fn describe_value_at_path(value: &Value, path: &str) -> String {
    let mut target = value;

    if !path.is_empty() {
        for segment in path.split('.') {
            let next = match target {
                Value::Object(map) => map.get(segment),
                Value::Array(items) => segment.parse::<usize>().ok().and_then(|index| items.get(index)),
                _ => None,
            };

            match next {
                Some(next) => target = next,
                None => return "nothing".to_owned(),
            }
        }
    }

    match target {
        Value::Null => "null".to_owned(),
        Value::Bool(_) => "boolean".to_owned(),
        Value::Number(_) => "number".to_owned(),
        Value::String(_) => "string".to_owned(),
        Value::Array(_) => "array".to_owned(),
        Value::Object(_) => "object".to_owned(),
    }
}

#[cfg(feature = "audit")]
fn record_undocumented_keys<T: DeserializeOwned>(endpoint: &str, value: &Value) {
    let _ = serde_ignored::deserialize::<_, _, T>(value, |path| {
        crate::core::audit::record_undocumented_key(endpoint, &path.to_string());
    });
}

/// Builds a [`Client`], rejecting a configuration that cannot work before the first request rather than as a puzzling
/// 401 an hour later.
#[derive(Default)]
pub struct ClientBuilder {
    host: Option<String>,
    auth: Option<Auth>,
    headers: Vec<(String, String)>,
    retry: RetryConfig,
    http: Option<reqwest::Client>,
    timeout: Option<Duration>,
    get_auth_on_401: Option<Arc<dyn AuthRefresher>>,
}

impl ClientBuilder {
    /// The bare site URL, e.g. `https://your-domain.atlassian.net`.
    ///
    /// Required for every strategy except Cloud OAuth 2.0 (3LO), whose tokens are not accepted on the site's own
    /// domain and which routes through `https://api.atlassian.com/ex/jira/{cloudId}` instead.
    #[must_use]
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    #[must_use]
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }

    /// A header sent with every request.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Opt-in retry for transient transport failures and 502/503/504. Disabled by default.
    #[must_use]
    pub fn retry(mut self, retry: RetryConfig) -> Self {
        self.retry = retry;
        self
    }

    /// The HTTP client every request goes through, including the OAuth 2.0 token and cloud-id calls.
    ///
    /// The one seam the transport offers: build it with a proxy, a timeout, a connection pool or a TLS configuration
    /// of your own.
    #[must_use]
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// A whole-request timeout, applied to the default HTTP client. Ignored when one is supplied.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Called once when the credentials are refused, to supply fresh ones and retry.
    ///
    /// Usually that is a 401. Not always: an endpoint permitting anonymous access answers `200` with an
    /// anonymous-scope body and reports the refusal only in `X-Seraph-LoginReason`, and this is called there too.
    #[must_use]
    pub fn get_auth_on_401(mut self, refresher: impl AuthRefresher) -> Self {
        self.get_auth_on_401 = Some(Arc::new(refresher));
        self
    }

    pub fn build(self) -> Result<Client> {
        if let Some(auth) = &self.auth {
            auth.validate()?;
        }

        let is_cloud_oauth = matches!(self.auth, Some(Auth::OAuth2(_)));

        let host = match self.host {
            Some(host) => {
                let parsed = url::Url::parse(&host)
                    .map_err(|error| Error::config(format!("`host` is not a valid URL: {error}")))?;

                if !matches!(parsed.scheme(), "http" | "https") {
                    return Err(Error::config("`host` must be an http or https URL."));
                }

                Some(host.trim_end_matches('/').to_owned())
            }
            None if is_cloud_oauth => None,
            None => {
                return Err(Error::config(
                    "`host` is required unless you authenticate with OAuth 2.0, which routes through the Atlassian \
gateway.",
                ));
            }
        };

        if matches!(self.auth, Some(Auth::OAuth2Server(_))) && host.is_none() {
            return Err(Error::config("Data Center OAuth 2.0 needs the instance it is talking to: pass `host`."));
        }

        let http = match self.http {
            Some(http) => http,
            None => {
                let mut builder = reqwest::Client::builder().user_agent(USER_AGENT);

                if let Some(timeout) = self.timeout {
                    builder = builder.timeout(timeout);
                }

                builder
                    .build()
                    .map_err(|error| Error::config(format!("The HTTP client could not be built: {error}")))?
            }
        };

        let oauth = match &self.auth {
            Some(Auth::OAuth2(config)) => Some(OAuth2Manager::cloud(config, http.clone())),
            Some(Auth::OAuth2Server(config)) => {
                Some(OAuth2Manager::server(config, host.clone().unwrap_or_default(), http.clone()))
            }
            _ => None,
        };

        Ok(Client {
            inner: Arc::new(Inner {
                http,
                host,
                auth: self.auth,
                headers: self.headers,
                retry: self.retry,
                oauth,
                get_auth_on_401: self.get_auth_on_401,
            }),
        })
    }
}

/// One request, built fluently.
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    client: Client,
    config: RequestConfig,
}

impl RequestBuilder {
    #[must_use]
    pub fn query(mut self, name: impl Into<String>, value: impl Into<QueryValue>) -> Self {
        self.config.query.push((name.into(), value.into()));
        self
    }

    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.headers.push((name.into(), value.into()));
        self
    }

    #[must_use]
    pub fn body(mut self, body: Body) -> Self {
        self.config.body = Some(body);
        self
    }

    /// Anything serialisable, as a JSON body.
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> Result<Self> {
        self.config.body =
            Some(Body::json(value).map_err(|error| {
                Error::config(format!("The request body could not be serialized as JSON: {error}"))
            })?);

        Ok(self)
    }

    #[must_use]
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.config.content_type = Some(content_type.into());
        self
    }

    pub async fn send<T: DeserializeOwned>(self) -> Result<T> {
        self.client.send(&self.config).await
    }

    pub async fn send_raw(self) -> Result<Value> {
        self.client.send_raw(&self.config).await
    }

    pub async fn send_bytes(self) -> Result<Bytes> {
        self.client.send_bytes(&self.config).await
    }

    pub async fn send_empty(self) -> Result<()> {
        self.client.send_empty(&self.config).await
    }
}

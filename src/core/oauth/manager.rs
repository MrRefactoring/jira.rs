use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::sync::Mutex;

use crate::core::auth::{OAuth2Config, OAuth2ServerConfig, TokenRefreshHook};
use crate::core::error::{Error, OAuthErrorDetails, Result};
use crate::core::oauth::helpers::{ACCESSIBLE_RESOURCES_URL, TOKEN_URL, get_accessible_resources_at};
use crate::core::oauth::server::{
    ServerRefreshTokenParams, normalize_host, refresh_cloud_token, refresh_server_oauth2_token,
};
use crate::core::oauth::types::{AccessibleResource, TokenRefreshEvent};
use crate::core::product::GATEWAY_SLUG;

/// Refresh this long before expiry, to absorb clock skew and in-flight latency.
const EXPIRY_SKEW: Duration = Duration::from_secs(60);

/// The Atlassian endpoints the Cloud flow talks to.
///
/// A field rather than a constant so the manager's own tests can answer for them; there is no way to set it from
/// outside the crate, because there is no other Atlassian to point it at.
#[derive(Debug, Clone)]
pub(crate) struct OAuthEndpoints {
    pub token_url: String,
    pub accessible_resources_url: String,
}

impl Default for OAuthEndpoints {
    fn default() -> Self {
        OAuthEndpoints {
            token_url: TOKEN_URL.to_owned(),
            accessible_resources_url: ACCESSIBLE_RESOURCES_URL.to_owned(),
        }
    }
}

/// Which deployment the tokens belong to, and what only that deployment needs.
#[derive(Debug, Clone)]
enum Deployment {
    /// Cloud 3LO: tokens are minted by `auth.atlassian.com` and accepted only through the Atlassian gateway, so the
    /// base URL is derived from a cloud id.
    Cloud { site_url: Option<String> },
    /// Data Center: the instance is its own authorization server and its own API host.
    Server { host: String, redirect_uri: Option<String> },
}

#[derive(Debug, Default)]
struct TokenState {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<SystemTime>,
    /// Bumped on every successful refresh, so a concurrent 401 can tell "refresh this" from "someone already did".
    generation: u64,
}

struct Inner {
    deployment: Deployment,
    endpoints: OAuthEndpoints,
    http: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
    on_token_refresh: Option<Arc<dyn TokenRefreshHook>>,
    tokens: Mutex<TokenState>,
    cloud_id: Mutex<Option<String>>,
}

/// Holds the OAuth 2.0 token state for one client: refreshes before expiry, resolves the cloud id once, and reports
/// rotated refresh tokens onwards.
///
/// Both the refresh and the cloud-id lookup are single-flighted by their mutexes, so N concurrent requests hitting an
/// expired token produce one token call, not N.
#[derive(Clone)]
pub(crate) struct OAuth2Manager {
    inner: Arc<Inner>,
}

impl OAuth2Manager {
    pub(crate) fn cloud(config: &OAuth2Config, http: reqwest::Client) -> Self {
        OAuth2Manager::cloud_at(config, http, OAuthEndpoints::default())
    }

    pub(crate) fn cloud_at(config: &OAuth2Config, http: reqwest::Client, endpoints: OAuthEndpoints) -> Self {
        OAuth2Manager {
            inner: Arc::new(Inner {
                deployment: Deployment::Cloud { site_url: config.site_url.clone() },
                endpoints,
                http,
                client_id: config.client_id.clone(),
                client_secret: config.client_secret.clone(),
                on_token_refresh: config.on_token_refresh.clone(),
                tokens: Mutex::new(TokenState {
                    access_token: config.access_token.clone(),
                    refresh_token: config.refresh_token.clone(),
                    expires_at: config.expires_at,
                    generation: 0,
                }),
                cloud_id: Mutex::new(config.cloud_id.clone()),
            }),
        }
    }

    pub(crate) fn server(config: &OAuth2ServerConfig, host: String, http: reqwest::Client) -> Self {
        OAuth2Manager {
            inner: Arc::new(Inner {
                deployment: Deployment::Server {
                    host: normalize_host(&host).to_owned(),
                    redirect_uri: config.redirect_uri.clone(),
                },
                endpoints: OAuthEndpoints::default(),
                http,
                client_id: config.client_id.clone(),
                client_secret: config.client_secret.clone(),
                on_token_refresh: config.on_token_refresh.clone(),
                tokens: Mutex::new(TokenState {
                    access_token: config.access_token.clone(),
                    refresh_token: config.refresh_token.clone(),
                    expires_at: config.expires_at,
                    generation: 0,
                }),
                cloud_id: Mutex::new(None),
            }),
        }
    }

    /// Whether a refresh is even possible — it needs the whole credential set.
    pub(crate) async fn can_refresh(&self) -> bool {
        let has_redirect_uri = match &self.inner.deployment {
            Deployment::Cloud { .. } => true,
            Deployment::Server { redirect_uri, .. } => redirect_uri.is_some(),
        };

        has_redirect_uri
            && self.inner.client_id.is_some()
            && self.inner.client_secret.is_some()
            && self.inner.tokens.lock().await.refresh_token.is_some()
    }

    /// `Bearer <token>`, refreshing first if the token is missing or within the skew window of expiry.
    ///
    /// The generation that comes back names the token that was handed out, so a 401 on this request can ask for a
    /// refresh without racing another request that already got one.
    pub(crate) async fn authorization_header(&self) -> Result<(String, u64)> {
        let mut tokens = self.inner.tokens.lock().await;

        if self.needs_refresh(&tokens).await {
            self.refresh_locked(&mut tokens).await?;
        }

        let token = tokens.access_token.clone().ok_or_else(|| {
            Error::oauth(
                "No OAuth 2.0 access token is available and it cannot be refreshed. Provide an `access_token`, or \
the full refresh credentials.",
                OAuthErrorDetails::default(),
            )
        })?;

        Ok((format!("Bearer {token}"), tokens.generation))
    }

    /// Refresh unless someone already did since `seen_generation` was handed out. Used by the 401 retry path.
    pub(crate) async fn force_refresh(&self, seen_generation: u64) -> Result<()> {
        let mut tokens = self.inner.tokens.lock().await;

        if tokens.generation > seen_generation {
            return Ok(());
        }

        self.refresh_locked(&mut tokens).await
    }

    /// The base URL every request goes to: the gateway for a resolved cloud id, or the instance itself.
    pub(crate) async fn base_url(&self) -> Result<String> {
        match &self.inner.deployment {
            Deployment::Server { host, .. } => Ok(host.clone()),
            Deployment::Cloud { .. } => {
                let cloud_id = self.resolve_cloud_id().await?;

                Ok(format!("https://api.atlassian.com/ex/{GATEWAY_SLUG}/{cloud_id}"))
            }
        }
    }

    async fn needs_refresh(&self, tokens: &TokenState) -> bool {
        if !self.has_refresh_credentials(tokens) {
            return false;
        }

        let Some(expires_at) = tokens.expires_at else {
            return tokens.access_token.is_none();
        };

        if tokens.access_token.is_none() {
            return true;
        }

        SystemTime::now() + EXPIRY_SKEW >= expires_at
    }

    fn has_refresh_credentials(&self, tokens: &TokenState) -> bool {
        let has_redirect_uri = match &self.inner.deployment {
            Deployment::Cloud { .. } => true,
            Deployment::Server { redirect_uri, .. } => redirect_uri.is_some(),
        };

        has_redirect_uri
            && self.inner.client_id.is_some()
            && self.inner.client_secret.is_some()
            && tokens.refresh_token.is_some()
    }

    async fn refresh_locked(&self, tokens: &mut TokenState) -> Result<()> {
        if !self.has_refresh_credentials(tokens) {
            return Err(Error::oauth(
                "Cannot refresh the OAuth 2.0 access token: the refresh token, client id and client secret are \
required, and a Data Center instance validates the redirect URI as well.",
                OAuthErrorDetails::default(),
            ));
        }

        let client_id = self.inner.client_id.clone().unwrap_or_default();
        let client_secret = self.inner.client_secret.clone().unwrap_or_default();
        let refresh_token = tokens.refresh_token.clone().unwrap_or_default();

        let response = match &self.inner.deployment {
            Deployment::Cloud { .. } => {
                refresh_cloud_token(
                    Some(&self.inner.http),
                    &self.inner.endpoints.token_url,
                    &client_id,
                    &client_secret,
                    &refresh_token,
                )
                .await?
            }
            Deployment::Server { host, redirect_uri } => {
                refresh_server_oauth2_token(&ServerRefreshTokenParams {
                    host: host.clone(),
                    client_id,
                    client_secret,
                    refresh_token,
                    redirect_uri: redirect_uri.clone().unwrap_or_default(),
                    http: Some(self.inner.http.clone()),
                })
                .await?
            }
        };

        let expires_at = response.expires_at();

        tokens.access_token = Some(response.access_token.clone());

        if let Some(rotated) = response.refresh_token.clone() {
            tokens.refresh_token = Some(rotated);
        }

        tokens.expires_at = Some(expires_at);
        tokens.generation += 1;

        if let Some(hook) = &self.inner.on_token_refresh {
            hook.on_token_refresh(TokenRefreshEvent {
                access_token: response.access_token,
                refresh_token: tokens.refresh_token.clone(),
                expires_at,
            })
            .await;
        }

        Ok(())
    }

    async fn resolve_cloud_id(&self) -> Result<String> {
        let mut cloud_id = self.inner.cloud_id.lock().await;

        if let Some(resolved) = cloud_id.as_ref() {
            return Ok(resolved.clone());
        }

        let resources = self.list_resources().await?;
        let resolved = self.select_resource(resources)?.id;

        *cloud_id = Some(resolved.clone());

        Ok(resolved)
    }

    /// The sites this token can reach, refreshing once if the token turns out to be stale.
    ///
    /// This lookup runs before the request loop, so the client's own 401-and-retry never covers it. Without this, a
    /// token whose expiry is unknown — the shape a caller supplying only an access token produces — would fail the
    /// cloud-id lookup permanently instead of refreshing the way any other request would.
    async fn list_resources(&self) -> Result<Vec<AccessibleResource>> {
        let (header, generation) = self.authorization_header().await?;
        let token = header.trim_start_matches("Bearer ").to_owned();

        let url = &self.inner.endpoints.accessible_resources_url;

        match get_accessible_resources_at(url, &token, Some(&self.inner.http)).await {
            Ok(resources) => Ok(resources),
            Err(error) => {
                if error.status() != Some(401) || !self.can_refresh().await {
                    return Err(error);
                }

                self.force_refresh(generation).await?;

                let (header, _) = self.authorization_header().await?;

                get_accessible_resources_at(url, header.trim_start_matches("Bearer "), Some(&self.inner.http)).await
            }
        }
    }

    fn select_resource(&self, resources: Vec<AccessibleResource>) -> Result<AccessibleResource> {
        if resources.is_empty() {
            return Err(Error::oauth(
                "No accessible resources were returned for this OAuth 2.0 token. Check the granted scopes and that \
the user has access to at least one site.",
                OAuthErrorDetails::default(),
            ));
        }

        let site_url = match &self.inner.deployment {
            Deployment::Cloud { site_url } => site_url.clone(),
            Deployment::Server { .. } => None,
        };

        let available = resources.iter().map(|resource| resource.url.clone()).collect::<Vec<_>>().join(", ");

        if let Some(site_url) = site_url {
            let target = normalize_site_url(&site_url);

            return resources.into_iter().find(|resource| normalize_site_url(&resource.url) == target).ok_or_else(
                || {
                    Error::oauth(
                        format!("No accessible resource matches siteUrl \"{site_url}\". Available: {available}."),
                        OAuthErrorDetails::default(),
                    )
                },
            );
        }

        if resources.len() > 1 {
            return Err(Error::oauth(
                format!(
                    "Multiple accessible resources found; pass `cloud_id` or `site_url` to disambiguate. \
Available: {available}."
                ),
                OAuthErrorDetails::default(),
            ));
        }

        Ok(resources.into_iter().next().expect("checked for emptiness above"))
    }
}

fn normalize_site_url(url: &str) -> String {
    url.trim_end_matches('/').to_lowercase()
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex as StdMutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn endpoints(server: &MockServer) -> OAuthEndpoints {
        OAuthEndpoints {
            token_url: format!("{}/oauth/token", server.uri()),
            accessible_resources_url: format!("{}/oauth/token/accessible-resources", server.uri()),
        }
    }

    fn refreshable(access_token: Option<&str>, expires_at: Option<SystemTime>) -> OAuth2Config {
        OAuth2Config {
            access_token: access_token.map(ToOwned::to_owned),
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("client".to_owned()),
            client_secret: Some("secret".to_owned()),
            expires_at,
            cloud_id: Some("cloud-1".to_owned()),
            ..OAuth2Config::default()
        }
    }

    async fn token_endpoint(server: &MockServer, body: serde_json::Value) {
        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(server)
            .await;
    }

    fn manager(config: &OAuth2Config, server: &MockServer) -> OAuth2Manager {
        OAuth2Manager::cloud_at(config, reqwest::Client::new(), endpoints(server))
    }

    #[tokio::test]
    async fn uses_the_access_token_as_given_when_it_is_not_near_expiry() {
        let server = MockServer::start().await;
        let config = refreshable(Some("fresh"), Some(SystemTime::now() + Duration::from_secs(3600)));

        let (header, generation) = manager(&config, &server).authorization_header().await.unwrap();

        assert_eq!(header, "Bearer fresh");
        assert_eq!(generation, 0);
        assert!(server.received_requests().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn refreshes_when_the_token_is_within_the_skew_window() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        // Thirty seconds of life left, which is inside the minute of skew the manager keeps.
        let config = refreshable(Some("stale"), Some(SystemTime::now() + Duration::from_secs(30)));

        let (header, generation) = manager(&config, &server).authorization_header().await.unwrap();

        assert_eq!(header, "Bearer minted");
        assert_eq!(generation, 1);
    }

    #[tokio::test]
    async fn refreshes_when_there_is_no_access_token_at_all() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        let (header, _) = manager(&refreshable(None, None), &server).authorization_header().await.unwrap();

        assert_eq!(header, "Bearer minted");
    }

    #[tokio::test]
    async fn does_not_refresh_what_it_has_no_credentials_to_refresh() {
        let server = MockServer::start().await;
        let config = OAuth2Config {
            access_token: Some("given".to_owned()),
            expires_at: Some(SystemTime::now() - Duration::from_secs(60)),
            cloud_id: Some("cloud-1".to_owned()),
            ..OAuth2Config::default()
        };
        let manager = manager(&config, &server);

        assert!(!manager.can_refresh().await);
        assert_eq!(manager.authorization_header().await.unwrap().0, "Bearer given");
    }

    #[tokio::test]
    async fn reports_a_rotated_refresh_token_so_it_can_be_persisted() {
        let server = MockServer::start().await;
        token_endpoint(
            &server,
            json!({
                "access_token": "minted",
                "refresh_token": "refresh-2",
                "expires_in": 3600,
                "token_type": "bearer",
            }),
        )
        .await;

        let seen = Arc::new(StdMutex::new(Vec::<String>::new()));
        let recorder = Arc::clone(&seen);
        let config = OAuth2Config {
            on_token_refresh: Some(Arc::new(move |event: TokenRefreshEvent| {
                let recorder = Arc::clone(&recorder);

                async move {
                    recorder.lock().unwrap().push(event.refresh_token.unwrap_or_default());
                }
            })),
            ..refreshable(None, None)
        };

        manager(&config, &server).authorization_header().await.unwrap();

        assert_eq!(seen.lock().unwrap().as_slice(), ["refresh-2"]);
    }

    #[tokio::test]
    async fn keeps_the_refresh_token_it_has_when_the_answer_does_not_rotate_one() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        let seen = Arc::new(StdMutex::new(Vec::<String>::new()));
        let recorder = Arc::clone(&seen);
        let config = OAuth2Config {
            on_token_refresh: Some(Arc::new(move |event: TokenRefreshEvent| {
                let recorder = Arc::clone(&recorder);

                async move {
                    recorder.lock().unwrap().push(event.refresh_token.unwrap_or_default());
                }
            })),
            ..refreshable(None, None)
        };

        manager(&config, &server).authorization_header().await.unwrap();

        assert_eq!(seen.lock().unwrap().as_slice(), ["refresh-1"]);
    }

    #[tokio::test]
    async fn single_flights_concurrent_refreshes_into_one_token_call() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        let manager = manager(&refreshable(None, None), &server);
        let waiting = (0..8).map(|_| {
            let manager = manager.clone();

            tokio::spawn(async move { manager.authorization_header().await })
        });

        for handle in waiting {
            assert_eq!(handle.await.unwrap().unwrap().0, "Bearer minted");
        }

        assert_eq!(server.received_requests().await.unwrap().len(), 1, "eight callers, one token call");
    }

    #[tokio::test]
    async fn force_refresh_does_nothing_when_someone_already_refreshed() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        let manager = manager(&refreshable(Some("stale"), None), &server);

        manager.force_refresh(0).await.unwrap();
        // A second caller holding the same stale generation must not burn another rotation.
        manager.force_refresh(0).await.unwrap();

        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn force_refresh_refreshes_again_for_a_caller_that_saw_the_newer_token() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        let manager = manager(&refreshable(Some("stale"), None), &server);

        manager.force_refresh(0).await.unwrap();
        manager.force_refresh(1).await.unwrap();

        assert_eq!(server.received_requests().await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn builds_the_gateway_base_url_from_the_cloud_id() {
        let server = MockServer::start().await;
        let base = manager(&refreshable(Some("fresh"), None), &server).base_url().await.unwrap();

        assert_eq!(base, "https://api.atlassian.com/ex/jira/cloud-1");
    }

    #[tokio::test]
    async fn resolves_the_cloud_id_once_and_remembers_it() {
        let server = MockServer::start().await;
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(move |_: &wiremock::Request| {
                counter.fetch_add(1, Ordering::SeqCst);

                ResponseTemplate::new(200)
                    .set_body_json(json!([{ "id": "cloud-9", "url": "https://acme.atlassian.net" }]))
            })
            .mount(&server)
            .await;

        let config = OAuth2Config { cloud_id: None, ..refreshable(Some("fresh"), None) };
        let manager = manager(&config, &server);

        assert_eq!(manager.base_url().await.unwrap(), "https://api.atlassian.com/ex/jira/cloud-9");
        assert_eq!(manager.base_url().await.unwrap(), "https://api.atlassian.com/ex/jira/cloud-9");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn picks_the_resource_that_matches_the_site_url() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                { "id": "cloud-a", "url": "https://one.atlassian.net" },
                { "id": "cloud-b", "url": "https://two.atlassian.net/" },
            ])))
            .mount(&server)
            .await;

        let config = OAuth2Config {
            cloud_id: None,
            site_url: Some("https://TWO.atlassian.net".to_owned()),
            ..refreshable(Some("fresh"), None)
        };

        assert_eq!(manager(&config, &server).base_url().await.unwrap(), "https://api.atlassian.com/ex/jira/cloud-b");
    }

    #[tokio::test]
    async fn refuses_to_guess_between_several_sites() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                { "id": "cloud-a", "url": "https://one.atlassian.net" },
                { "id": "cloud-b", "url": "https://two.atlassian.net" },
            ])))
            .mount(&server)
            .await;

        let config = OAuth2Config { cloud_id: None, ..refreshable(Some("fresh"), None) };
        let error = manager(&config, &server).base_url().await.unwrap_err();

        assert!(error.is_oauth());
        assert!(error.to_string().contains("disambiguate"), "{error}");
    }

    #[tokio::test]
    async fn says_so_when_the_site_url_matches_nothing() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                { "id": "cloud-a", "url": "https://one.atlassian.net" },
            ])))
            .mount(&server)
            .await;

        let config = OAuth2Config {
            cloud_id: None,
            site_url: Some("https://other.atlassian.net".to_owned()),
            ..refreshable(Some("fresh"), None)
        };
        let error = manager(&config, &server).base_url().await.unwrap_err();

        assert!(error.to_string().contains("No accessible resource matches"), "{error}");
    }

    #[tokio::test]
    async fn says_so_when_the_token_reaches_no_site_at_all() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .mount(&server)
            .await;

        let config = OAuth2Config { cloud_id: None, ..refreshable(Some("fresh"), None) };
        let error = manager(&config, &server).base_url().await.unwrap_err();

        assert!(error.to_string().contains("No accessible resources"), "{error}");
    }

    #[tokio::test]
    async fn refreshes_once_when_the_cloud_id_lookup_answers_401() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "error": "unauthorized" })))
            .up_to_n_times(1)
            .with_priority(1)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([{ "id": "cloud-9", "url": "https://acme.atlassian.net" }])),
            )
            .with_priority(2)
            .mount(&server)
            .await;

        // No expiry, so nothing says the token is stale until the lookup refuses it.
        let config = OAuth2Config { cloud_id: None, ..refreshable(Some("stale"), None) };

        assert_eq!(manager(&config, &server).base_url().await.unwrap(), "https://api.atlassian.com/ex/jira/cloud-9");
    }

    #[tokio::test]
    async fn gives_up_on_the_cloud_id_lookup_when_there_is_nothing_to_refresh_with() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "error": "unauthorized" })))
            .mount(&server)
            .await;

        let config = OAuth2Config { access_token: Some("stale".to_owned()), cloud_id: None, ..OAuth2Config::default() };
        let error = manager(&config, &server).base_url().await.unwrap_err();

        assert_eq!(error.status(), Some(401));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn does_not_loop_when_the_refreshed_token_is_rejected_too() {
        let server = MockServer::start().await;
        token_endpoint(&server, json!({ "access_token": "minted", "expires_in": 3600, "token_type": "bearer" })).await;

        Mock::given(method("GET"))
            .and(path("/oauth/token/accessible-resources"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "error": "unauthorized" })))
            .mount(&server)
            .await;

        let config = OAuth2Config { cloud_id: None, ..refreshable(Some("stale"), None) };
        let error = manager(&config, &server).base_url().await.unwrap_err();

        assert_eq!(error.status(), Some(401));

        let lookups = server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|request| request.url.path().ends_with("accessible-resources"))
            .count();

        assert_eq!(lookups, 2, "one attempt, one retry after the refresh, and no more");
    }

    #[tokio::test]
    async fn flags_a_dead_grant_as_needing_a_fresh_authorization() {
        for code in ["invalid_grant", "unauthorized_client"] {
            let server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/oauth/token"))
                .respond_with(ResponseTemplate::new(400).set_body_json(json!({ "error": code })))
                .mount(&server)
                .await;

            let error = manager(&refreshable(None, None), &server).authorization_header().await.unwrap_err();

            assert!(error.is_reauthorization_required(), "{code}");
            assert_eq!(error.oauth_code(), Some(code));
        }
    }

    #[tokio::test]
    async fn does_not_flag_access_denied_from_the_token_endpoint_because_that_is_a_bad_client_secret() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({ "error": "access_denied" })))
            .mount(&server)
            .await;

        let error = manager(&refreshable(None, None), &server).authorization_header().await.unwrap_err();

        assert!(error.is_oauth());
        assert!(!error.is_reauthorization_required());
    }

    #[tokio::test]
    async fn survives_a_non_json_body_from_the_auth_server() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/oauth/token"))
            .respond_with(ResponseTemplate::new(502).set_body_raw("<html>gateway</html>", "text/html"))
            .mount(&server)
            .await;

        let error = manager(&refreshable(None, None), &server).authorization_header().await.unwrap_err();

        assert!(error.is_oauth());
        assert_eq!(error.status(), Some(502));
        assert!(!error.is_reauthorization_required());
    }
}

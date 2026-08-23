use std::time::{Duration, SystemTime};

use jira::core::oauth::AuthorizationUrlParams;
use jira::core::oauth::{
    ServerAuthorizationUrlParams, ServerExchangeCodeParams, ServerOAuth2Scope, ServerRefreshTokenParams,
    exchange_server_authorization_code, generate_authorization_url, generate_server_authorization_url,
    refresh_server_oauth2_token,
};
use jira::core::{Auth, Client, OAuth2Config, OAuth2ServerConfig};
use serde::Deserialize;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Deserialize)]
struct Myself {
    #[serde(rename = "displayName")]
    display_name: String,
}

#[test]
fn builds_the_cloud_authorization_url() {
    let url = generate_authorization_url(&AuthorizationUrlParams::new(
        "client-1",
        ["read:jira-work", "offline_access"],
        "https://app.example.com/cb",
        "state-1",
    ));

    assert!(url.starts_with("https://auth.atlassian.com/authorize?"), "{url}");
    assert!(url.contains("audience=api.atlassian.com"), "{url}");
    assert!(url.contains("client_id=client-1"), "{url}");
    assert!(url.contains("scope=read%3Ajira-work+offline_access"), "{url}");
    assert!(url.contains("response_type=code"), "{url}");
    assert!(url.contains("prompt=consent"), "{url}");
}

#[test]
fn builds_the_data_center_authorization_url_on_the_instance_itself() {
    let url = generate_server_authorization_url(&ServerAuthorizationUrlParams {
        host: "https://jira.acme.internal/".to_owned(),
        client_id: "link-1".to_owned(),
        scopes: vec![ServerOAuth2Scope::Read, ServerOAuth2Scope::Write],
        redirect_uri: "https://app.example.com/cb".to_owned(),
        state: "state-1".to_owned(),
    });

    assert!(url.starts_with("https://jira.acme.internal/rest/oauth2/latest/authorize?"), "{url}");
    assert!(url.contains("scope=READ+WRITE"), "{url}");
    assert!(!url.contains("audience"), "a self-hosted instance has no gateway audience: {url}");
}

#[tokio::test]
async fn exchanges_a_data_center_code_as_a_form_not_as_json() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "minted",
            "refresh_token": "refresh-2",
            "expires_in": 3600,
            "token_type": "bearer",
        })))
        .mount(&server)
        .await;

    let tokens = exchange_server_authorization_code(&ServerExchangeCodeParams {
        host: server.uri(),
        client_id: "link-1".to_owned(),
        client_secret: "secret".to_owned(),
        code: "code-1".to_owned(),
        redirect_uri: "https://app.example.com/cb".to_owned(),
        http: None,
    })
    .await
    .unwrap();

    assert_eq!(tokens.access_token, "minted");
    assert_eq!(tokens.refresh_token.as_deref(), Some("refresh-2"));

    let body = String::from_utf8_lossy(&server.received_requests().await.unwrap()[0].body).into_owned();

    assert!(body.contains("grant_type=authorization_code"), "{body}");
    assert!(body.contains("client_secret=secret"), "{body}");
}

#[tokio::test]
async fn sends_the_redirect_uri_on_the_refresh_grant_too() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "minted",
            "expires_in": 3600,
            "token_type": "bearer",
        })))
        .mount(&server)
        .await;

    refresh_server_oauth2_token(&ServerRefreshTokenParams {
        host: server.uri(),
        client_id: "link-1".to_owned(),
        client_secret: "secret".to_owned(),
        refresh_token: "refresh-1".to_owned(),
        redirect_uri: "https://app.example.com/cb".to_owned(),
        http: None,
    })
    .await
    .unwrap();

    let body = String::from_utf8_lossy(&server.received_requests().await.unwrap()[0].body).into_owned();

    assert!(body.contains("redirect_uri="), "the provider validates it on refresh as well: {body}");
}

#[tokio::test]
async fn reports_what_the_instance_said_when_the_grant_is_refused() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "invalid_grant",
            "error_description": "The refresh token is invalid",
        })))
        .mount(&server)
        .await;

    let error = refresh_server_oauth2_token(&ServerRefreshTokenParams {
        host: server.uri(),
        client_id: "link-1".to_owned(),
        client_secret: "secret".to_owned(),
        refresh_token: "dead".to_owned(),
        redirect_uri: "https://app.example.com/cb".to_owned(),
        http: None,
    })
    .await
    .unwrap_err();

    assert!(error.is_reauthorization_required());
    assert_eq!(error.oauth_code(), Some("invalid_grant"));
}

#[tokio::test]
async fn a_data_center_client_refreshes_before_expiry_and_sends_the_new_token() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "minted",
            "refresh_token": "refresh-2",
            "expires_in": 3600,
            "token_type": "bearer",
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/rest/api/2/myself"))
        .and(header("authorization", "Bearer minted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::oauth2_server(OAuth2ServerConfig {
            access_token: Some("stale".to_owned()),
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("link-1".to_owned()),
            client_secret: Some("secret".to_owned()),
            redirect_uri: Some("https://app.example.com/cb".to_owned()),
            // Already expired, so the first request refreshes before it goes out.
            expires_at: Some(SystemTime::now() - Duration::from_secs(10)),
            ..OAuth2ServerConfig::default()
        }))
        .build()
        .unwrap();

    let myself: Myself = client.get("/rest/api/2/myself").send().await.unwrap();

    assert_eq!(myself.display_name, "Ada");
}

#[tokio::test]
async fn a_data_center_client_refreshes_once_on_a_401_and_retries() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "minted",
            "expires_in": 3600,
            "token_type": "bearer",
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/rest/api/2/myself"))
        .and(header("authorization", "Bearer stale"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "message": "no" })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/rest/api/2/myself"))
        .and(header("authorization", "Bearer minted"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::oauth2_server(OAuth2ServerConfig {
            access_token: Some("stale".to_owned()),
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("link-1".to_owned()),
            client_secret: Some("secret".to_owned()),
            redirect_uri: Some("https://app.example.com/cb".to_owned()),
            ..OAuth2ServerConfig::default()
        }))
        .build()
        .unwrap();

    let myself: Myself = client.get("/rest/api/2/myself").send().await.unwrap();

    assert_eq!(myself.display_name, "Ada");
}

#[tokio::test]
async fn a_data_center_client_does_not_loop_when_the_refreshed_token_is_refused_too() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/rest/oauth2/latest/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "minted",
            "expires_in": 3600,
            "token_type": "bearer",
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/rest/api/2/myself"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "message": "no" })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::oauth2_server(OAuth2ServerConfig {
            access_token: Some("stale".to_owned()),
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("link-1".to_owned()),
            client_secret: Some("secret".to_owned()),
            redirect_uri: Some("https://app.example.com/cb".to_owned()),
            ..OAuth2ServerConfig::default()
        }))
        .build()
        .unwrap();

    let error = client.get("/rest/api/2/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());

    let calls = server.received_requests().await.unwrap();
    let api_calls = calls.iter().filter(|request| request.url.path() == "/rest/api/2/myself").count();

    assert_eq!(api_calls, 2, "one attempt, one retry after the refresh, and no more");
}

#[test]
fn cloud_oauth_may_leave_the_host_out_because_it_routes_through_the_gateway() {
    let client = Client::builder()
        .auth(Auth::oauth2(OAuth2Config { access_token: Some("token".to_owned()), ..OAuth2Config::default() }))
        .build()
        .unwrap();

    assert_eq!(client.host(), None);
}

#[test]
fn data_center_oauth_needs_the_instance_it_is_talking_to() {
    let error = Client::builder()
        .auth(Auth::oauth2_server(OAuth2ServerConfig {
            access_token: Some("token".to_owned()),
            ..OAuth2ServerConfig::default()
        }))
        .build()
        .unwrap_err();

    assert!(error.is_config());
}

#[test]
fn rejects_a_half_configured_refresh_credential_set() {
    // The access token alone would be a valid configuration. Half a refresh set alongside it is always a mistake: it
    // looks configured, then fails on the first refresh.
    let error = Client::builder()
        .auth(Auth::oauth2(OAuth2Config {
            access_token: Some("token".to_owned()),
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("client".to_owned()),
            ..OAuth2Config::default()
        }))
        .build()
        .unwrap_err();

    assert!(error.is_config());
    assert!(error.to_string().contains("must all be provided together"), "{error}");
}

#[test]
fn rejects_a_refresh_set_that_cannot_authenticate_now_either() {
    let error = Client::builder()
        .auth(Auth::oauth2(OAuth2Config {
            refresh_token: Some("refresh-1".to_owned()),
            client_id: Some("client".to_owned()),
            ..OAuth2Config::default()
        }))
        .build()
        .unwrap_err();

    assert!(error.is_config());
    assert!(error.to_string().contains("full refresh credential set"), "{error}");
}

#[test]
fn rejects_credentials_that_can_neither_authenticate_nor_refresh() {
    let error = Client::builder().auth(Auth::oauth2(OAuth2Config::default())).build().unwrap_err();

    assert!(error.is_config());
    assert!(error.to_string().contains("either an `accessToken`"), "{error}");
}

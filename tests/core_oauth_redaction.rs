use std::time::{Duration, SystemTime};

use jira::core::oauth::{
    CallbackParams, ExchangeCodeParams, RefreshTokenParams, ServerExchangeCodeParams, ServerRefreshTokenParams,
    TokenRefreshEvent, TokenResponse,
};

const SECRET: &str = "s3cr3t-nobody-should-read";

fn leaks(rendered: &str) -> bool {
    rendered.contains(SECRET)
}

#[test]
fn exchange_code_params_print_neither_the_secret_nor_the_code() {
    let params = ExchangeCodeParams::new("client-1", SECRET, SECRET, "https://example.test/callback");

    let rendered = format!("{params:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("client-1"));
    assert!(rendered.contains("https://example.test/callback"));
}

#[test]
fn refresh_token_params_print_neither_the_secret_nor_the_refresh_token() {
    let params = RefreshTokenParams::new("client-1", SECRET, SECRET);

    let rendered = format!("{params:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("client-1"));
}

#[test]
fn server_exchange_code_params_print_neither_the_secret_nor_the_code() {
    let params = ServerExchangeCodeParams {
        host: "https://jira.example.test".to_owned(),
        client_id: "client-1".to_owned(),
        client_secret: SECRET.to_owned(),
        code: SECRET.to_owned(),
        redirect_uri: "https://example.test/callback".to_owned(),
        http: None,
    };

    let rendered = format!("{params:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("https://jira.example.test"));
}

#[test]
fn server_refresh_token_params_print_neither_the_secret_nor_the_refresh_token() {
    let params = ServerRefreshTokenParams {
        host: "https://jira.example.test".to_owned(),
        client_id: "client-1".to_owned(),
        client_secret: SECRET.to_owned(),
        refresh_token: SECRET.to_owned(),
        redirect_uri: "https://example.test/callback".to_owned(),
        http: None,
    };

    let rendered = format!("{params:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("https://jira.example.test"));
}

#[test]
fn a_token_response_prints_neither_token() {
    let response = TokenResponse {
        access_token: SECRET.to_owned(),
        refresh_token: Some(SECRET.to_owned()),
        expires_in: 3600,
        scope: "read:jira-work".to_owned(),
        token_type: "bearer".to_owned(),
    };

    let rendered = format!("{response:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains(r#"refresh_token: Some("<redacted>")"#), "{rendered}");
    assert!(rendered.contains("read:jira-work"));
    assert!(rendered.contains("3600"));
}

#[test]
fn a_token_response_without_a_refresh_token_says_so_rather_than_redacting_nothing() {
    let response = TokenResponse {
        access_token: SECRET.to_owned(),
        refresh_token: None,
        expires_in: 3600,
        scope: String::new(),
        token_type: "bearer".to_owned(),
    };

    let rendered = format!("{response:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("refresh_token: None"), "{rendered}");
    assert!(!rendered.contains(r#"refresh_token: Some"#), "{rendered}");
}

#[test]
fn the_event_handed_to_the_refresh_hook_prints_neither_token() {
    let event = TokenRefreshEvent {
        access_token: SECRET.to_owned(),
        refresh_token: Some(SECRET.to_owned()),
        expires_at: SystemTime::now() + Duration::from_secs(3600),
    };

    let rendered = format!("{event:?}");

    assert!(!leaks(&rendered), "{rendered}");
}

#[test]
fn the_callback_prints_the_state_but_not_the_code() {
    let params = CallbackParams { code: SECRET.to_owned(), state: "state-1".to_owned() };

    let rendered = format!("{params:?}");

    assert!(!leaks(&rendered), "{rendered}");
    assert!(rendered.contains("state-1"));
}

use std::time::{Duration, SystemTime};

use jira::core::{ApiErrorKind, Error, create_api_error, parse_retry_after};
use serde_json::json;

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

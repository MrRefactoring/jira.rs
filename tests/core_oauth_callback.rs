use jira::core::oauth::parse_callback_url;

#[test]
fn returns_the_code_when_state_matches() {
    let parsed = parse_callback_url("https://app.example.com/cb?code=abc&state=xyz", "xyz").unwrap();

    assert_eq!(parsed.code, "abc");
    assert_eq!(parsed.state, "xyz");
}

#[test]
fn accepts_a_relative_url_as_frameworks_hand_it_over() {
    let parsed = parse_callback_url("/cb?code=abc&state=xyz", "xyz").unwrap();

    assert_eq!(parsed.code, "abc");
}

#[test]
fn reports_a_declined_consent_as_access_denied() {
    let error =
        parse_callback_url("https://app.example.com/cb?error=access_denied&error_description=User+said+no", "xyz")
            .unwrap_err();

    assert!(error.is_oauth());
    assert_eq!(error.oauth_code(), Some("access_denied"));
    assert!(error.is_reauthorization_required());
    assert!(error.to_string().contains("User said no"), "{error}");
}

#[test]
fn reports_any_other_error_code_from_the_callback() {
    let error = parse_callback_url("https://app.example.com/cb?error=server_error", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("server_error"));
    assert!(!error.is_reauthorization_required());
}

#[test]
fn rejects_a_mismatched_state_before_looking_at_the_code() {
    let error = parse_callback_url("https://app.example.com/cb?code=abc&state=nope", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("invalid_state"));
}

#[test]
fn rejects_a_missing_state() {
    let error = parse_callback_url("https://app.example.com/cb?code=abc", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("invalid_state"));
}

#[test]
fn rejects_a_callback_carrying_neither_code_nor_error() {
    let error = parse_callback_url("https://app.example.com/cb?state=xyz", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("invalid_request"));
}

#[test]
fn checks_the_error_before_the_state_so_a_decline_is_not_reported_as_tampering() {
    let error = parse_callback_url("https://app.example.com/cb?error=access_denied&state=nope", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("access_denied"));
}

#[test]
fn a_state_of_a_different_length_never_matches() {
    let error = parse_callback_url("https://app.example.com/cb?code=abc&state=xy", "xyz").unwrap_err();

    assert_eq!(error.oauth_code(), Some("invalid_state"));
}

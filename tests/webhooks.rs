#![cfg(feature = "webhooks")]

use jira::webhooks::{WebhookEvent, WebhookPayload, verify_signature};

const SECRET: &str = "shhh";
const ISSUE_CREATED: &str = r#"{"webhookEvent":"jira:issue_created"}"#;
const ISSUE_CREATED_SIGNATURE: &str = "sha256=3fabe2d04ab39e6c32cd69443906602514ec29b8c013897d29901059b6adfbce";

#[test]
fn accepts_a_body_signed_with_the_same_secret() {
    let trusted = verify_signature(ISSUE_CREATED.as_bytes(), SECRET, Some(ISSUE_CREATED_SIGNATURE))
        .expect("a non-empty secret is usable");

    assert!(trusted, "the body carries the digest this secret produces");
}

#[test]
fn rejects_a_body_altered_after_it_was_signed() {
    let altered = r#"{"webhookEvent":"jira:issue_deleted"}"#;

    let trusted = verify_signature(altered.as_bytes(), SECRET, Some(ISSUE_CREATED_SIGNATURE)).unwrap();

    assert!(!trusted, "the digest was computed over different bytes");
}

#[test]
fn rejects_the_right_body_signed_with_a_different_secret() {
    let elsewhere = "sha256=c4bccb3286420357e625bdb5f756837637f48cfb713bbfedfba880719e71f3ae";

    let trusted = verify_signature(ISSUE_CREATED.as_bytes(), SECRET, Some(elsewhere)).unwrap();

    assert!(!trusted, "anyone can sign a body; only this secret signs it this way");
}

#[test]
fn agrees_with_rfc_4231_so_the_algorithm_is_the_one_jira_uses() {
    let expected = "sha256=5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843";

    let trusted = verify_signature(b"what do ya want for nothing?", "Jefe", Some(expected)).unwrap();

    assert!(trusted, "test case 2 of RFC 4231 verifies, so this is HMAC-SHA256 and not something adjacent");
}

#[test]
fn answers_false_for_an_unsigned_delivery_rather_than_making_the_caller_branch() {
    assert!(!verify_signature(b"{}", SECRET, None).unwrap());
}

#[test]
fn rejects_a_signature_it_cannot_trust() {
    let refused = [
        ("an algorithm we do not accept", "sha1=aabb"),
        ("a digest with no algorithm", "aabbccdd"),
        ("a digest that is not hexadecimal", "sha256=zzzz"),
        ("a digest of odd length", "sha256=abc"),
        ("an empty digest", "sha256="),
        ("an empty header", ""),
        ("a digest a sign would sneak past a radix parser", "sha256=+abc"),
    ];

    for (what, signature) in refused {
        assert!(!verify_signature(b"{}", SECRET, Some(signature)).unwrap(), "{what} was accepted");
    }
}

#[test]
fn an_empty_secret_is_a_mistake_of_the_callers_rather_than_a_failed_check() {
    let refused = verify_signature(ISSUE_CREATED.as_bytes(), "", Some(ISSUE_CREATED_SIGNATURE));

    assert!(refused.is_err(), "an empty secret would verify every body ever sent");
}

#[test]
fn reads_a_documented_issue_payload() {
    let body = r#"{
        "timestamp": 1709189449954,
        "webhookEvent": "jira:issue_created",
        "issue_event_type_name": "issue_created",
        "matchedWebhookIds": [1, 2],
        "issue": { "id": "10001", "key": "TEST-1" },
        "user": { "accountId": "5b10a2844c20165700ede21g" }
    }"#;

    let payload: WebhookPayload = serde_json::from_str(body).expect("a documented payload reads");

    assert_eq!(payload.webhook_event, Some(WebhookEvent::JiraIssueCreated));
    assert_eq!(payload.issue_event_type_name.as_deref(), Some("issue_created"));
    assert_eq!(payload.issue.and_then(|issue| issue.key), Some("TEST-1".to_owned()));
    assert_eq!(payload.matched_webhook_ids, Some(vec![1, 2]));
}

#[test]
fn an_event_atlassian_added_and_did_not_write_down_still_reads() {
    let body = r#"{"webhookEvent":"something_atlassian_added_last_tuesday","timestamp":1}"#;

    let payload: WebhookPayload = serde_json::from_str(body).expect("an unknown event is not a parse failure");

    assert_eq!(
        payload.webhook_event.expect("the event is there").as_str(),
        "something_atlassian_added_last_tuesday",
        "a webhook receiver that rejects a new event is an outage rather than a safeguard",
    );
}

#[test]
fn an_entity_the_payload_does_not_carry_is_absent_rather_than_a_failure() {
    let body = r#"{"webhookEvent":"sprint_started","timestamp":1}"#;

    let payload: WebhookPayload = serde_json::from_str(body).unwrap();

    assert_eq!(payload.webhook_event, Some(WebhookEvent::SprintStarted));
    assert!(payload.sprint.is_none(), "Atlassian documents none of this, so every entity is optional");
}

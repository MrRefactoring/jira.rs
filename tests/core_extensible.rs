use jira::Extensible;
use jira::cloud::IssueFields;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Estimation {
    #[serde(rename = "customfield_10016", skip_serializing_if = "Option::is_none")]
    story_points: Option<f64>,
    #[serde(rename = "customfield_10020", skip_serializing_if = "Option::is_none")]
    sprint: Option<String>,
}

fn fields() -> IssueFields {
    IssueFields { summary: Some("Rotate the signing key".to_owned()), ..IssueFields::default() }
}

#[test]
fn a_custom_type_round_trips_through_the_undescribed_keys() {
    let written = fields()
        .with_custom(Estimation { story_points: Some(5.0), sprint: Some("Sprint 3".to_owned()) })
        .expect("custom fields are accepted");

    let read: Estimation = written.custom().expect("the custom fields read back");

    assert_eq!(read, Estimation { story_points: Some(5.0), sprint: Some("Sprint 3".to_owned()) });
}

#[test]
fn custom_keys_sit_beside_the_described_fields_on_the_wire() {
    let written = fields().with("customfield_10016", 5.0).expect("a custom key is accepted");

    let body = serde_json::to_value(&written).expect("fields serialize");

    assert_eq!(body, json!({ "summary": "Rotate the signing key", "customfield_10016": 5.0 }));
}

#[test]
fn custom_keys_that_arrived_are_read_the_same_way() {
    let received: IssueFields =
        serde_json::from_value(json!({ "summary": "x", "customfield_10016": 3.0 })).expect("fields deserialize");

    let read: Estimation = received.custom().expect("the custom fields read back");

    assert_eq!(read, Estimation { story_points: Some(3.0), sprint: None });
}

#[test]
fn a_key_the_schema_describes_is_refused_rather_than_written_twice() {
    let error = fields().with("summary", "another").expect_err("a described key is refused");

    assert!(error.to_string().contains("`summary`"), "{error}");

    let error = fields().with("fixVersions", json!([])).expect_err("a described key is refused by its wire name");

    assert!(error.to_string().contains("`fixVersions`"), "{error}");
}

#[test]
fn custom_fields_that_are_not_an_object_are_refused() {
    let error = fields().with_custom("not an object").expect_err("a scalar cannot be spread into keys");

    assert!(error.to_string().contains("JSON object"), "{error}");
}

#[test]
fn a_key_added_twice_keeps_the_later_value() {
    let written = fields()
        .with("customfield_10016", 1.0)
        .and_then(|fields| fields.with("customfield_10016", 2.0))
        .expect("a custom key is accepted");

    assert_eq!(written.additional()["customfield_10016"], 2.0);
}

#[test]
fn the_described_keys_are_the_wire_names() {
    assert!(IssueFields::FIELDS.contains(&"summary"));
    assert!(IssueFields::FIELDS.contains(&"fixVersions"));
    assert!(!IssueFields::FIELDS.contains(&"fix_versions"));
    assert!(!IssueFields::FIELDS.contains(&"additional"));
}

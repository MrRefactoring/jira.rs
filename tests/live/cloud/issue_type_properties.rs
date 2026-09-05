//! Ported from jira.js/tests/live/cloud/issueTypeProperties.test.ts.
//!
//! The fourth home of the same entity-property mechanism, after issues, projects and users. Exercised in full under a
//! namespaced key.
//!
//! This one carries a caveat the others do not: an issue type is site-wide, so the property is visible to every
//! project that uses that type. It is still safe — a key nothing else reads affects nothing — but it is not
//! project-scoped, and a suite that treated it as such would be reasoning about the wrong blast radius.

use serde_json::json;

use crate::harness::{ResourceTracker, TEST_ISSUE_TYPE, TEST_PROJECT_KEY, await_readable, await_refused, cloud};

const PROPERTY_KEY: &str = "jira.rs.livetest.issuetype";

/// The property round trip, end to end: stored, listed, replaced, deleted, and unreadable afterwards.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_an_issue_type_property_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue_type_id = test_issue_type_id().await;
    let value = json!({ "scope": "issue type", "list": [1, 2, 3] });

    set_property(&mut tracker, &issue_type_id, &value).await;

    let property = cloud()
        .issue_type_properties()
        .get_issue_type_property(&issue_type_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the property reads back");

    assert_eq!(property.key.as_deref(), Some(PROPERTY_KEY));
    assert_eq!(property.value, Some(value), "the stored value survives the round trip untouched");

    let keys = cloud()
        .issue_type_properties()
        .get_issue_type_property_keys(&issue_type_id)
        .send()
        .await
        .expect("the issue type lists its property keys")
        .keys
        .unwrap_or_default();

    let entry = keys.iter().find(|entry| entry.key.as_deref() == Some(PROPERTY_KEY)).expect("the stored key is listed");

    assert!(
        entry.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a listed key links to where it can be read: {:?}",
        entry.self_,
    );

    cloud()
        .issue_type_properties()
        .set_issue_type_property(&issue_type_id, PROPERTY_KEY, object_of(&json!({ "only": "this" })))
        .send()
        .await
        .expect("the property can be written a second time");

    let replaced = await_readable("the rewritten property reads back", || {
        cloud().issue_type_properties().get_issue_type_property(&issue_type_id, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(replaced.value, Some(json!({ "only": "this" })), "a second write replaces the value, it does not merge");

    cloud()
        .issue_type_properties()
        .delete_issue_type_property(&issue_type_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the property can be deleted");

    let error = await_refused("a deleted property cannot be read", || {
        cloud().issue_type_properties().get_issue_type_property(&issue_type_id, PROPERTY_KEY).send()
    })
    .await;

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

/// The blast radius, pinned: the type is shared with every project that offers it, the property space is not.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn keeps_the_issue_type_property_out_of_the_project_property_space() {
    let mut tracker = ResourceTracker::new();
    let issue_type_id = test_issue_type_id().await;

    set_property(&mut tracker, &issue_type_id, &json!({ "scope": "issue type" })).await;

    let statuses = cloud()
        .projects()
        .get_all_statuses(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project lists the statuses of the types it offers");

    assert!(!statuses.is_empty(), "the test project offers issue types the property could have leaked onto");

    let error = cloud()
        .project_properties()
        .get_project_property(TEST_PROJECT_KEY, PROPERTY_KEY)
        .send()
        .await
        .expect_err("the key set on the issue type is not a property of the project");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_property_on_an_unknown_issue_type_as_a_typed_error() {
    let error = cloud()
        .issue_type_properties()
        .get_issue_type_property("99999999", PROPERTY_KEY)
        .send()
        .await
        .expect_err("an issue type that does not exist has no properties");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

fn object_of(value: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    value.as_object().expect("a property value is an object").clone().into_iter().collect()
}

/// The id of the type the suite's fixtures use, read from the test project rather than assumed.
async fn test_issue_type_id() -> String {
    cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back by key")
        .issue_types
        .unwrap_or_default()
        .into_iter()
        .find(|issue_type| issue_type.name.as_deref() == Some(TEST_ISSUE_TYPE))
        .and_then(|issue_type| issue_type.id)
        .expect("the test project offers the issue type its fixtures use")
}

/// Writes the property and registers its removal, so a test that fails mid-way still leaves the type as it found it.
async fn set_property(tracker: &mut ResourceTracker, issue_type_id: &str, value: &serde_json::Value) {
    cloud()
        .issue_type_properties()
        .set_issue_type_property(issue_type_id, PROPERTY_KEY, object_of(value))
        .send()
        .await
        .expect("the issue type takes a property");

    let owner = issue_type_id.to_owned();

    tracker.defer(move || {
        let owner = owner.clone();

        async move { cloud().issue_type_properties().delete_issue_type_property(owner, PROPERTY_KEY).send().await }
    });
}

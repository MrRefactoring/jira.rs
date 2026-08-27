//! Ported from jira.js/tests/live/cloud/projectProperties.test.ts.
//!
//! The same entity-property mechanism as `issue_properties`, hung off a project instead. Exercised in full because it
//! is scoped to a namespaced key nothing else reads, and removable.
//!
//! Worth a suite of its own rather than trusting the issue-property one by analogy: these are four separate generated
//! functions against four separate URLs, and "it works for issues" has never been evidence that it works for projects.

use serde_json::json;

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, await_readable, await_refused, cloud};

const PROPERTY_KEY: &str = "jira.rs.livetest.project";

fn object_of(value: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    value.as_object().expect("a property value is an object").clone().into_iter().collect()
}

/// The project property round trip, end to end.
///
/// Proves that a nested value comes back byte for byte, that the listing links to what it names, that a second write
/// replaces rather than merges, that the project can be addressed by id as well as by key, and that a deleted
/// property is unreadable.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_project_property_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let value = json!({ "nested": { "list": [1, "two", null] }, "flag": true });

    cloud()
        .project_properties()
        .set_project_property(TEST_PROJECT_KEY, PROPERTY_KEY, object_of(&value))
        .send()
        .await
        .expect("the project takes a property");

    tracker.defer(|| async {
        cloud().project_properties().delete_project_property(TEST_PROJECT_KEY, PROPERTY_KEY).send().await
    });

    let property = await_readable("the property reads back", || {
        cloud().project_properties().get_project_property(TEST_PROJECT_KEY, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(property.key.as_deref(), Some(PROPERTY_KEY));
    assert_eq!(property.value, Some(value), "a nested, mixed-type value survives the round trip untouched");

    let listed = cloud()
        .project_properties()
        .get_project_property_keys(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the listing sees the stored property");

    let entry = listed
        .keys
        .unwrap_or_default()
        .into_iter()
        .find(|key| key.key.as_deref() == Some(PROPERTY_KEY))
        .expect("the stored key is listed");

    assert!(
        entry.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a listed key links to where it can be read: {:?}",
        entry.self_,
    );

    cloud()
        .project_properties()
        .set_project_property(TEST_PROJECT_KEY, PROPERTY_KEY, object_of(&json!({ "only": "this" })))
        .send()
        .await
        .expect("the property can be written a second time");

    let replaced = await_readable("the rewritten property reads back", || {
        cloud().project_properties().get_project_property(TEST_PROJECT_KEY, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(replaced.value, Some(json!({ "only": "this" })), "a second write replaces the value, it does not merge");

    let project = await_readable("the test project reads back by key", || {
        cloud().projects().get_project(TEST_PROJECT_KEY).send()
    })
    .await;
    let id = project.id.expect("a project carries an id");

    let by_id = await_readable("the property reads back with the project addressed by id", || {
        cloud().project_properties().get_project_property(&id, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(by_id.value, Some(json!({ "only": "this" })), "id and key address the same property store");

    cloud()
        .project_properties()
        .delete_project_property(TEST_PROJECT_KEY, PROPERTY_KEY)
        .send()
        .await
        .expect("the property can be deleted");

    let error = await_refused("a deleted property cannot be read", || {
        cloud().project_properties().get_project_property(TEST_PROJECT_KEY, PROPERTY_KEY).send()
    })
    .await;

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_property_on_a_missing_project_as_not_found() {
    let error = cloud()
        .project_properties()
        .get_project_property("NOSUCHPROJECT", PROPERTY_KEY)
        .send()
        .await
        .expect_err("a project that does not exist has no properties");

    assert!(error.is_not_found(), "{error}");
}

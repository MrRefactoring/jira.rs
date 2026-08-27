//! Ported from jira.js/tests/live/cloud/issueProperties.test.ts.
//!
//! Entity properties are arbitrary JSON hung off an issue, which makes them the cleanest write surface in the whole
//! API — scoped to a fixture issue, removable, and with no side effects on anything Jira renders. So the round trip
//! is exercised in full, including the part that is easy to get wrong: a nested value must survive verbatim, not
//! flattened or stringified on the way through.

use jira::cloud::{BulkIssuePropertyUpdateRequest, IssueFilterForBulkPropertySet};
use serde_json::json;

use crate::harness::{
    ResourceTracker, TEST_PROJECT_KEY, await_readable, await_refused, cloud, create_test_issue, poll_until, test_name,
};

const PROPERTY_KEY: &str = "jira.rs.livetest";

/// Deliberately nested and mixed-type — a serializer that mangles anything will show up here.
fn value() -> serde_json::Value {
    json!({
        "nested": { "deep": [1, 2, { "flag": true }] },
        "text": "значение",
        "count": 42,
        "nothing": null,
    })
}

fn object_of(value: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    value.as_object().expect("a property value is an object").clone().into_iter().collect()
}

/// The property round trip, end to end.
///
/// Proves that a fresh issue carries none, that a nested value comes back byte for byte, that the listing links to
/// what it names, that a second write replaces rather than merges, and that a deleted property is gone from both the
/// direct read and the listing.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_property_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("properties"))).await;

    let empty = cloud()
        .issue_properties()
        .get_issue_property_keys(&issue.key)
        .send()
        .await
        .expect("a fresh issue lists its properties");

    assert!(empty.keys.unwrap_or_default().is_empty(), "a fresh issue carries no properties");

    cloud()
        .issue_properties()
        .set_issue_property(&issue.key, PROPERTY_KEY, object_of(&value()))
        .send()
        .await
        .expect("the issue takes a property");

    let property = await_readable("the property reads back", || {
        cloud().issue_properties().get_issue_property(&issue.key, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(property.key.as_deref(), Some(PROPERTY_KEY));
    assert_eq!(property.value, Some(value()), "a nested, mixed-type value survives the round trip untouched");

    let listed = cloud()
        .issue_properties()
        .get_issue_property_keys(&issue.key)
        .send()
        .await
        .expect("the listing sees the stored property");

    let keys = listed.keys.expect("the listing carries its keys");

    assert!(keys.iter().any(|entry| entry.key.as_deref() == Some(PROPERTY_KEY)), "the stored key is listed");
    assert!(
        keys[0].self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a listed key links to where it can be read: {:?}",
        keys[0].self_,
    );

    cloud()
        .issue_properties()
        .set_issue_property(&issue.key, PROPERTY_KEY, object_of(&json!({ "only": "this" })))
        .send()
        .await
        .expect("the property can be written a second time");

    let replaced = await_readable("the rewritten property reads back", || {
        cloud().issue_properties().get_issue_property(&issue.key, PROPERTY_KEY).send()
    })
    .await;

    assert_eq!(replaced.value, Some(json!({ "only": "this" })), "a second write replaces the value, it does not merge");

    cloud()
        .issue_properties()
        .delete_issue_property(&issue.key, PROPERTY_KEY)
        .send()
        .await
        .expect("the property can be deleted");

    let error = await_refused("a deleted property cannot be read", || {
        cloud().issue_properties().get_issue_property(&issue.key, PROPERTY_KEY).send()
    })
    .await;

    assert!(error.is_not_found(), "{error}");

    poll_until("the deleted property to leave the listing", || async {
        let remaining = cloud()
            .issue_properties()
            .get_issue_property_keys(&issue.key)
            .send()
            .await
            .expect("the listing reads after a delete");

        (!remaining.keys.unwrap_or_default().iter().any(|entry| entry.key.as_deref() == Some(PROPERTY_KEY)))
            .then_some(())
    })
    .await;

    tracker.cleanup().await;
}

/// One property, several issues, one call — and the write lands asynchronously, so the reads wait for it.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn sets_one_property_across_several_issues_in_a_single_bulk_call() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("bulk property one"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("bulk property two"))).await;

    let ids =
        [&first, &second].iter().map(|issue| issue.id.parse().expect("an issue id is a number")).collect::<Vec<i64>>();

    cloud()
        .issue_properties()
        .bulk_set_issue_property(
            PROPERTY_KEY,
            BulkIssuePropertyUpdateRequest {
                value: Some(json!({ "bulk": true })),
                filter: Some(IssueFilterForBulkPropertySet {
                    entity_ids: Some(ids),
                    ..IssueFilterForBulkPropertySet::default()
                }),
                ..BulkIssuePropertyUpdateRequest::default()
            },
        )
        .send()
        .await
        .expect("a bulk property write is accepted");

    for key in [&first.key, &second.key] {
        let value = poll_until("the bulk write to land", || async {
            cloud()
                .issue_properties()
                .get_issue_property(key, PROPERTY_KEY)
                .send()
                .await
                .ok()
                .and_then(|property| property.value)
        })
        .await;

        assert_eq!(value, json!({ "bulk": true }), "the one call set the property on every issue it named");
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_property_on_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_properties()
        .get_issue_property(format!("{TEST_PROJECT_KEY}-99999999"), PROPERTY_KEY)
        .send()
        .await
        .expect_err("an issue that does not exist has no properties");

    assert!(error.is_not_found(), "{error}");
}

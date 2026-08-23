//! Ported from jira.js/tests/live/cloud/commentAndWorklogProperties.test.ts.
//!
//! The last two homes of the entity-property mechanism, after issues, projects, users and issue types. Both are
//! exercised in full: their parents are fixture objects this suite created, so the whole cycle is contained.
//!
//! Grouped in one file because the point is the comparison. Six modules share this mechanism, and the interesting
//! question is no longer "does it store JSON" — it is whether the six namespaces are genuinely separate, which is
//! asserted here across the two that hang off the shortest-lived parents.

use jira::cloud::{CommentInput, CommentInputBody, WorklogInput};
use serde_json::json;

use crate::harness::{ResourceTracker, cloud, create_test_issue, document_of, test_name};

const PROPERTY_KEY: &str = "jira.rs.livetest.child";

fn object_of(value: &serde_json::Value) -> std::collections::HashMap<String, serde_json::Value> {
    value.as_object().expect("a property value is an object").clone().into_iter().collect()
}

/// Comment and worklog properties, walked side by side.
///
/// Proves each parent stores and returns its own value, that the two namespaces — and the issue's own — do not leak
/// into one another, that each parent lists its key, and that deleting one leaves the other untouched.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_comment_and_worklog_properties_side_by_side() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("child properties"))).await;

    let comment = cloud()
        .issue_comments()
        .add_comment(
            &issue.key,
            CommentInput {
                body: Some(CommentInputBody::Document(document_of("property carrier"))),
                ..CommentInput::default()
            },
        )
        .send()
        .await
        .expect("the issue takes a comment");

    let comment_id = comment.id.expect("a created comment carries an id");

    let worklog = cloud()
        .issue_worklogs()
        .add_worklog(&issue.key, WorklogInput { time_spent: Some("10m".to_owned()), ..WorklogInput::default() })
        .send()
        .await
        .expect("the issue takes a worklog");

    let worklog_id = worklog.id.expect("a created worklog carries an id");

    cloud()
        .issue_comment_properties()
        .set_comment_property(&comment_id, PROPERTY_KEY, object_of(&json!({ "on": "comment" })))
        .send()
        .await
        .expect("the comment takes a property");

    let on_comment = cloud()
        .issue_comment_properties()
        .get_comment_property(&comment_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the comment property reads back");

    assert_eq!(on_comment.key.as_deref(), Some(PROPERTY_KEY));
    assert_eq!(on_comment.value, Some(json!({ "on": "comment" })));

    cloud()
        .issue_worklog_properties()
        .set_worklog_property(&issue.key, &worklog_id, PROPERTY_KEY, object_of(&json!({ "on": "worklog" })))
        .send()
        .await
        .expect("the worklog takes a property");

    let on_worklog = cloud()
        .issue_worklog_properties()
        .get_worklog_property(&issue.key, &worklog_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the worklog property reads back");

    assert_eq!(on_worklog.value, Some(json!({ "on": "worklog" })));

    let comment_again = cloud()
        .issue_comment_properties()
        .get_comment_property(&comment_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the comment property is still its own");

    assert_eq!(comment_again.value, Some(json!({ "on": "comment" })), "the worklog write did not reach the comment");

    let on_issue = cloud()
        .issue_properties()
        .get_issue_property(&issue.key, PROPERTY_KEY)
        .send()
        .await
        .expect_err("the same key on the parent issue was never written");

    assert!(on_issue.is_not_found(), "{on_issue}");

    let comment_keys = cloud()
        .issue_comment_properties()
        .get_comment_property_keys(&comment_id)
        .send()
        .await
        .expect("the comment lists its property keys");

    assert!(
        comment_keys.keys.unwrap_or_default().iter().any(|entry| entry.key.as_deref() == Some(PROPERTY_KEY)),
        "the comment lists the key it stores",
    );

    let worklog_keys = cloud()
        .issue_worklog_properties()
        .get_worklog_property_keys(&issue.key, &worklog_id)
        .send()
        .await
        .expect("the worklog lists its property keys");

    assert!(
        worklog_keys.keys.unwrap_or_default().iter().any(|entry| entry.key.as_deref() == Some(PROPERTY_KEY)),
        "the worklog lists the key it stores",
    );

    cloud()
        .issue_comment_properties()
        .delete_comment_property(&comment_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the comment property can be deleted");

    let gone_from_comment = cloud()
        .issue_comment_properties()
        .get_comment_property(&comment_id, PROPERTY_KEY)
        .send()
        .await
        .expect_err("a deleted comment property cannot be read");

    assert!(gone_from_comment.is_not_found(), "{gone_from_comment}");

    let still_on_worklog = cloud()
        .issue_worklog_properties()
        .get_worklog_property(&issue.key, &worklog_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the worklog property outlives the comment's");

    assert_eq!(still_on_worklog.value, Some(json!({ "on": "worklog" })), "the two deletes are independent");

    cloud()
        .issue_worklog_properties()
        .delete_worklog_property(&issue.key, &worklog_id, PROPERTY_KEY)
        .send()
        .await
        .expect("the worklog property can be deleted");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_property_on_a_missing_comment_as_a_typed_error() {
    let error = cloud()
        .issue_comment_properties()
        .get_comment_property("99999999", PROPERTY_KEY)
        .send()
        .await
        .expect_err("a comment that does not exist has no properties");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

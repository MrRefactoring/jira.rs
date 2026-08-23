use jira::cloud::{CreatedIssue, Document, IssueUpdateDetails};
use serde_json::json;

use super::client::cloud;
use super::naming::test_name;
use super::resources::ResourceTracker;

/// The project every Cloud suite works in. Its issue types are `Task` and `Sub-task`.
///
/// Issues are created in an existing project rather than a fresh one: creating a Jira project is slow, consumes a
/// licence slot, and often cannot be deleted cleanly by the same token that made it. A dedicated test project is the
/// cheaper and more reliable unit of isolation, and run-scoped names keep concurrent runs apart inside it.
pub const TEST_PROJECT_KEY: &str = "AUTOTEST";

/// The issue type used unless a suite needs something else.
pub const TEST_ISSUE_TYPE: &str = "Task";

/// A minimal ADF document wrapping one line of text.
pub fn document_of(text: &str) -> Document {
    serde_json::from_value(json!({
        "type": "doc",
        "version": 1,
        "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": text }] }],
    }))
    .expect("a hand-built ADF paragraph is a document")
}

/// Creates an issue in the test project and registers its deletion.
pub async fn create_test_issue(tracker: &mut ResourceTracker, summary: Option<&str>) -> CreatedIssue {
    create_issue_with(
        tracker,
        json!({
            "project": { "key": TEST_PROJECT_KEY },
            "issuetype": { "name": TEST_ISSUE_TYPE },
            "summary": summary.map_or_else(|| test_name("issue"), ToOwned::to_owned),
        }),
    )
    .await
}

/// Creates an issue from the fields given, and registers its deletion.
pub async fn create_issue_with(tracker: &mut ResourceTracker, fields: serde_json::Value) -> CreatedIssue {
    let fields = fields
        .as_object()
        .expect("issue fields are an object")
        .iter()
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();

    let created = cloud()
        .issues()
        .create_issue(IssueUpdateDetails {
            fields: Some(fields),
            ..IssueUpdateDetails::default()
        })
        .send()
        .await
        .expect("the test project accepts a new issue");

    let key = created.key.clone();

    tracker.defer(move || {
        let key = key.clone();

        async move { cloud().issues().delete_issue(key).send().await }
    });

    created
}

//! Ported from jira.js/tests/live/cloud/issueNavigatorSettings.test.ts.
//!
//! The one endpoint here writes site-wide state: these are the columns every account sees in the issue navigator
//! until it sets its own. The suite therefore reads the current list first and registers its restoration on the
//! tracker before writing anything, so the site ends the run exactly as it started it.
//!
//! Worth its own file because the write is easy to get wrong in a way that looks like a transport fault. Atlassian
//! declares the body only under a wildcard media type, which generated a shapeless object; Jira answers a bare array
//! with 400 and a form-encoded body with 415, and accepts exactly `{ "columns": [...] }` as JSON.

use jira::cloud::ColumnRequestBody;

use crate::harness::{ResourceTracker, await_readable, cloud};

/// Read, replace, read back — and put the site's own list back on the way out.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn replaces_the_default_navigator_columns_and_puts_them_back() {
    let mut tracker = ResourceTracker::new();

    let original = cloud()
        .issue_navigator_settings()
        .get_issue_navigator_default_columns()
        .send()
        .await
        .expect("the site reports the columns the issue navigator defaults to");

    assert!(!original.is_empty(), "a site always has default navigator columns");

    for column in &original {
        assert!(
            column.value.as_deref().is_some_and(|value| !value.is_empty()),
            "a column is addressed by a value: {column:?}"
        );
        assert!(
            column.label.as_deref().is_some_and(|label| !label.is_empty()),
            "a column is shown under a label: {column:?}"
        );
    }

    let restore: Vec<String> = original.iter().filter_map(|column| column.value.clone()).collect();

    tracker.defer(move || {
        let restore = restore.clone();

        async move {
            cloud()
                .issue_navigator_settings()
                .set_issue_navigator_default_columns(ColumnRequestBody { columns: Some(restore) })
                .send()
                .await
        }
    });

    cloud()
        .issue_navigator_settings()
        .set_issue_navigator_default_columns(ColumnRequestBody {
            columns: Some(vec!["summary".to_owned(), "status".to_owned()]),
        })
        .send()
        .await
        .expect("the default columns can be replaced");

    let replaced = await_readable("the replacement reads back", || {
        cloud().issue_navigator_settings().get_issue_navigator_default_columns().send()
    })
    .await;

    assert_eq!(
        replaced.iter().filter_map(|column| column.value.clone()).collect::<Vec<String>>(),
        vec!["summary".to_owned(), "status".to_owned()],
        "the write replaced the list rather than adding to it",
    );

    tracker.cleanup().await;
}

/// A rejected write changes nothing, which is why this one needs no restoration.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_column_that_does_not_exist() {
    let error = cloud()
        .issue_navigator_settings()
        .set_issue_navigator_default_columns(ColumnRequestBody { columns: Some(vec!["nosuchcolumn".to_owned()]) })
        .send()
        .await
        .expect_err("a column that is not a navigable field cannot be a default");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

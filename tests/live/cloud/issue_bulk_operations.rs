//! Ported from jira.js/tests/live/cloud/issueBulkOperations.test.ts.
//!
//! These are the only endpoints in the platform API that are genuinely asynchronous: a submit answers with a task id
//! and the work happens later. That shape is the whole point of the suite — a caller who treats the 2xx as "done"
//! will read stale data immediately afterwards and have no idea why.
//!
//! Writes are confined to fixture issues, and the reversible operations are preferred: watch leaves nothing behind,
//! while bulk delete is only ever submitted with an empty selection, which Jira refuses. Bulk change is also a global
//! permission a site can withhold, so the reads stand down on a typed refusal rather than failing the run.

use jira::cloud::{BulkOperationProgressStatus, IssueBulkDeletePayload, IssueBulkWatchOrUnwatchPayload};

use crate::harness::{ResourceTracker, cloud, create_test_issue, poll_until, test_name};

/// Both bulk reads over one pair of issues: what can be edited across them, and what they can all transition to.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_fields_and_transitions_a_pair_of_issues_has_in_common() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("bulk fields one"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("bulk fields two"))).await;
    let pair = format!("{},{}", first.key, second.key);

    match cloud().issue_bulk_operations().get_bulk_editable_fields(&pair).send().await {
        Ok(page) => {
            let fields = page.fields.unwrap_or_default();

            assert!(!fields.is_empty(), "two issues of the same type have fields that can be edited together");
            assert!(
                fields.iter().all(|field| field.id.as_deref().is_some_and(|id| !id.is_empty())),
                "every editable field is named by an id: {fields:?}",
            );
        }
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused bulk read is typed: {error}")
        }
    }

    match cloud().issue_bulk_operations().get_available_transitions(&pair).send().await {
        Ok(available) => {
            let workflows = available.available_transitions.unwrap_or_default();

            assert!(
                workflows.iter().all(|workflow| workflow.transitions.is_some() && workflow.issues.is_some()),
                "a workflow group names both the issues it covers and the transitions they share: {workflows:?}",
            );
        }
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused bulk read is typed: {error}")
        }
    }

    tracker.cleanup().await;
}

/// The asynchronous contract: the submit answers with a task, and the effect appears afterwards.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_a_bulk_watch_with_a_task_and_applies_it_afterwards() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("bulk watch one"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("bulk watch two"))).await;

    let submitted = cloud()
        .issue_bulk_operations()
        .submit_bulk_watch(IssueBulkWatchOrUnwatchPayload {
            selected_issue_ids_or_keys: vec![first.key.clone(), second.key.clone()],
        })
        .send()
        .await;

    let submitted = match submitted {
        Ok(submitted) => submitted,
        // Bulk change is a global permission a site can withhold; the refusal is still typed, which is the part the
        // library owns.
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused bulk submit is typed: {error}");

            tracker.cleanup().await;

            return;
        }
    };

    let task_id = submitted.task_id.expect("a submitted bulk operation answers with a task id rather than a result");

    assert!(!task_id.is_empty(), "the task id is what the caller has to come back with");

    let progress = cloud()
        .issue_bulk_operations()
        .get_bulk_operation_progress(&task_id)
        .send()
        .await
        .expect("the task the submit named can be asked about");

    assert_eq!(progress.task_id.as_deref(), Some(task_id.as_str()), "the progress report names the task it is for");
    assert!(
        progress.status.as_ref().is_some_and(BulkOperationProgressStatus::is_documented),
        "the task reports a status the specification names: {:?}",
        progress.status,
    );

    let account_id = cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site says who the token speaks for")
        .account_id
        .expect("an account carries an id");

    let watchers = poll_until("the accepted watch to be applied", || async {
        cloud().issue_watchers().get_issue_watchers(&first.key).send().await.ok().filter(|watchers| {
            watchers
                .watchers
                .as_deref()
                .unwrap_or_default()
                .iter()
                .any(|watcher| watcher.account_id.as_deref() == Some(account_id.as_str()))
        })
    })
    .await;

    assert_eq!(watchers.is_watching, Some(true), "the account that submitted the bulk watch is watching the issue");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_task_as_a_typed_error() {
    let error =
        cloud().tasks().get_task("99999999").send().await.expect_err("a task that does not exist cannot be read");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_bulk_request_naming_no_issues() {
    let error = cloud()
        .issue_bulk_operations()
        .submit_bulk_delete(IssueBulkDeletePayload {
            selected_issue_ids_or_keys: Vec::new(),
            ..IssueBulkDeletePayload::default()
        })
        .send()
        .await
        .expect_err("a bulk delete that names no issues is not a request Jira accepts");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

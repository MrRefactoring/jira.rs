//! Ported from jira.js/tests/live/cloud/issueWatchers.test.ts.
//!
//! Scoped entirely to fixture issues and to the authenticating account — no other user is ever added as a watcher, so
//! the suite cannot generate mail for a real person.
//!
//! Note the asymmetry the API forces on callers: the watcher is *added* as a JSON body and *removed* as a query
//! parameter. The body is a bare JSON string — the account id, quoted, on its own — which is why `add_watcher` takes
//! a plain string rather than a model.

use jira::cloud::IssueList;

use crate::harness::{
    ResourceTracker, TEST_PROJECT_KEY, await_readable, cloud, create_test_issue, poll_until, test_name,
};

async fn current_account_id() -> String {
    cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site knows the caller")
        .account_id
        .expect("an authenticated user has an account id")
}

/// The watcher lifecycle, end to end.
///
/// Proves the list is self-consistent, that an add is observable and idempotent, that the bulk query agrees with the
/// per-issue read, and that a removal through the query parameter actually removes.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_watcher_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("watchers"))).await;
    let account_id = current_account_id().await;

    let fresh = cloud()
        .issue_watchers()
        .get_issue_watchers(&issue.key)
        .send()
        .await
        .expect("a fresh issue reports its watchers");

    assert!(
        fresh.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "the watcher list carries its own URL: {:?}",
        fresh.self_,
    );
    assert!(fresh.is_watching.is_some(), "the watcher list says whether the caller is watching");
    assert_eq!(
        fresh.watch_count,
        fresh.watchers.as_ref().map(|watchers| i64::try_from(watchers.len()).unwrap_or(i64::MAX)),
        "the count matches the list it counts",
    );

    cloud()
        .issue_watchers()
        .add_watcher(&issue.key, &account_id)
        .send()
        .await
        .expect("the calling account can watch the issue");

    let key = issue.key.clone();
    let watcher = account_id.clone();

    tracker.defer(move || {
        let (key, watcher) = (key.clone(), watcher.clone());

        async move { cloud().issue_watchers().remove_watcher(key).account_id(watcher).send().await }
    });

    let watching = poll_until("the calling account to appear in the watcher list", || async {
        let watching =
            await_readable("the watchers read back", || cloud().issue_watchers().get_issue_watchers(&issue.key).send())
                .await;

        let listed = watching
            .watchers
            .iter()
            .flatten()
            .any(|watcher| watcher.account_id.as_deref() == Some(account_id.as_str()));

        listed.then_some(watching)
    })
    .await;

    assert_eq!(watching.is_watching, Some(true), "the add is observable on the next read");

    cloud().issue_watchers().add_watcher(&issue.key, &account_id).send().await.expect("a repeated add is accepted");

    let again =
        await_readable("the watchers read back", || cloud().issue_watchers().get_issue_watchers(&issue.key).send())
            .await;

    assert_eq!(again.watch_count, watching.watch_count, "a repeated add is idempotent rather than cumulative");

    let bulk = cloud()
        .issue_watchers()
        .get_is_watching_issue_bulk(IssueList { issue_ids: vec![issue.id.clone()] })
        .send()
        .await
        .expect("the bulk watching query answers");

    assert_eq!(
        bulk.issues_is_watching
            .as_ref()
            .and_then(|watching| watching.get(&issue.id))
            .and_then(serde_json::Value::as_bool),
        Some(true),
        "the bulk query agrees with the per-issue read",
    );

    cloud()
        .issue_watchers()
        .remove_watcher(&issue.key)
        .account_id(&account_id)
        .send()
        .await
        .expect("the watcher can be removed through the query parameter");

    let removed = poll_until("the removed account to leave the watcher list", || async {
        let removed =
            await_readable("the watchers read back", || cloud().issue_watchers().get_issue_watchers(&issue.key).send())
                .await;

        let gone = !removed
            .watchers
            .iter()
            .flatten()
            .any(|watcher| watcher.account_id.as_deref() == Some(account_id.as_str()));

        gone.then_some(removed)
    })
    .await;

    assert_eq!(removed.is_watching, Some(false));

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_unknown_account_id_rather_than_ignoring_it() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("unknown watcher"))).await;

    let error = cloud()
        .issue_watchers()
        .add_watcher(&issue.key, "no-such-account-id")
        .send()
        .await
        .expect_err("an account that does not exist cannot watch an issue");

    assert!(
        error.status().is_some_and(|status| (400..500).contains(&status)),
        "an unknown account is the caller's mistake, not the site's: {error}",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_watchers_of_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_watchers()
        .get_issue_watchers(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist has no watchers");

    assert!(error.is_not_found(), "{error}");
}

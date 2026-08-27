//! Ported from jira.js/tests/live/cloud/issueWorklogs.test.ts.
//!
//! A full write cycle inside fixture issues. Two things here are worth a live site rather than a unit test: time is
//! expressed as a human string that Jira parses server-side into seconds, and every worklog write mutates the issue's
//! time-tracking totals — a side effect nothing in the return value mentions.

use std::time::{SystemTime, UNIX_EPOCH};

use jira::cloud::{Worklog, WorklogIdsRequest, WorklogInput, WorklogInputComment};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, document_of, poll_until, test_name};

fn worklog_of(time_spent: &str, comment: Option<&str>) -> WorklogInput {
    WorklogInput {
        time_spent: Some(time_spent.to_owned()),
        comment: comment.map(|text| WorklogInputComment::Document(document_of(text))),
        ..WorklogInput::default()
    }
}

/// Logs time against the issue and registers the worklog's deletion.
async fn add_worklog(
    tracker: &mut ResourceTracker,
    issue_key: &str,
    time_spent: &str,
    comment: Option<&str>,
) -> Worklog {
    let created = cloud()
        .issue_worklogs()
        .add_worklog(issue_key, worklog_of(time_spent, comment))
        .send()
        .await
        .expect("the issue takes a worklog");

    let key = issue_key.to_owned();
    let id = created.id.clone().expect("a created worklog carries an id");

    tracker.defer(move || {
        let (key, id) = (key.clone(), id.clone());

        async move { cloud().issue_worklogs().delete_worklog(key, id).send().await }
    });

    created
}

/// The issue's own tally of logged seconds, as the `timetracking` field reports it.
async fn time_spent_seconds(issue_key: &str) -> Option<i64> {
    cloud()
        .issues()
        .get_issue(issue_key)
        .fields(["timetracking"])
        .send()
        .await
        .expect("the issue reads back with its time tracking")
        .fields
        .as_ref()
        .and_then(|fields| fields.get("timetracking"))
        .and_then(|tracking| tracking.get("timeSpentSeconds"))
        .and_then(serde_json::Value::as_i64)
}

/// The worklog lifecycle, end to end.
///
/// The assertions that matter are the two the caller cannot see coming: `1h 30m` is parsed into 5400 seconds by the
/// server rather than by the client, and each write moves the issue's time-tracking total, which no response body
/// mentions.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_worklog_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("worklogs"))).await;

    let empty =
        cloud().issue_worklogs().get_issue_worklog(&issue.key).send().await.expect("a fresh issue lists worklogs");

    assert_eq!(empty.total, Some(0));
    assert!(empty.worklogs.is_none_or(|worklogs| worklogs.is_empty()), "a fresh issue carries no worklogs");

    let created = add_worklog(&mut tracker, &issue.key, "1h 30m", Some("worked on it")).await;
    let worklog_id = created.id.clone().expect("a created worklog carries an id");

    assert!(worklog_id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {worklog_id}");
    assert_eq!(created.time_spent_seconds, Some(5400), "the server parses `1h 30m` into seconds");
    assert_eq!(created.time_spent.as_deref(), Some("1h 30m"));
    assert!(
        created.author.as_ref().and_then(|author| author.account_id.as_deref()).is_some_and(|id| !id.is_empty()),
        "a worklog carries the account that logged it",
    );
    assert!(
        created.started.as_deref().is_some_and(|started| started.contains('T')),
        "a worklog starts at an ISO 8601 instant: {:?}",
        created.started,
    );

    assert_eq!(time_spent_seconds(&issue.key).await, Some(5400), "the logged time lands on the issue total");

    let fetched = cloud()
        .issue_worklogs()
        .get_worklog(&issue.key, &worklog_id)
        .send()
        .await
        .expect("the worklog reads back by id");

    assert_eq!(fetched.id.as_deref(), Some(worklog_id.as_str()));
    assert_eq!(fetched.issue_id.as_deref(), Some(issue.id.as_str()));

    let comment = serde_json::to_string(&fetched.comment).expect("a worklog comment is serialisable");

    assert!(comment.contains("worked on it"), "{comment}");

    let updated = cloud()
        .issue_worklogs()
        .update_worklog(&issue.key, &worklog_id, worklog_of("2h", None))
        .send()
        .await
        .expect("the duration can be changed");

    assert_eq!(updated.time_spent_seconds, Some(7200));
    assert_eq!(time_spent_seconds(&issue.key).await, Some(7200), "the issue total moves with the worklog");

    cloud().issue_worklogs().delete_worklog(&issue.key, &worklog_id).send().await.expect("the worklog can be deleted");

    poll_until("the worklog listing to reflect the delete", || async {
        let listing = cloud()
            .issue_worklogs()
            .get_issue_worklog(&issue.key)
            .send()
            .await
            .expect("the listing reads after a delete");

        (listing.total == Some(0)).then_some(listing)
    })
    .await;

    let error = cloud()
        .issue_worklogs()
        .get_worklog(&issue.key, &worklog_id)
        .send()
        .await
        .expect_err("a deleted worklog cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_worklog_listing() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("paged worklogs"))).await;

    add_worklog(&mut tracker, &issue.key, "1h 30m", Some("worked on it")).await;
    add_worklog(&mut tracker, &issue.key, "15m", None).await;

    let all = cloud().issue_worklogs().get_issue_worklog(&issue.key).send().await.expect("the worklog list");

    assert_eq!(all.total, Some(2));

    let limited = cloud()
        .issue_worklogs()
        .get_issue_worklog(&issue.key)
        .max_results(1)
        .send()
        .await
        .expect("`maxResults` is accepted");

    assert_eq!(limited.worklogs.as_ref().map(Vec::len), Some(1));
    assert_eq!(limited.max_results, Some(1));

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fetches_worklogs_by_id_across_issues() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("worklogs by id"))).await;
    let created = add_worklog(&mut tracker, &issue.key, "1h 30m", Some("worked on it")).await;
    let worklog_id = created.id.clone().expect("a created worklog carries an id");

    let worklogs = cloud()
        .issue_worklogs()
        .get_worklogs_for_ids(WorklogIdsRequest { ids: vec![worklog_id.parse().expect("a worklog id is a number")] })
        .send()
        .await
        .expect("worklogs can be fetched by id");

    assert_eq!(worklogs.len(), 1);
    assert_eq!(worklogs[0].id.as_deref(), Some(worklog_id.as_str()));

    tracker.cleanup().await;
}

/// The modified-since feed is a site-wide cursor, so it is asserted on its own invariants rather than on a fixture.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_worklogs_modified_since_a_point_in_the_past() {
    let now =
        i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH).expect("the clock is past the epoch").as_millis())
            .expect("a millisecond timestamp fits in an i64");
    let since = now - 60 * 60 * 1000;

    let page = cloud()
        .issue_worklogs()
        .get_ids_of_worklogs_modified_since()
        .since(since)
        .send()
        .await
        .expect("the modified-since feed answers");

    assert!(page.values.is_some(), "the feed carries a list of entries, empty or not");
    assert!(page.last_page.is_some(), "the feed says whether this is the last page");
    assert!(
        page.until.is_some_and(|until| until >= since),
        "the cursor cannot end before it started: {:?}",
        page.until
    );

    for entry in page.values.into_iter().flatten() {
        assert!(
            entry.updated_time.is_some_and(|updated| updated >= since),
            "an entry cannot predate the cursor: {:?}",
            entry.updated_time,
        );
        assert!(entry.worklog_id.is_some_and(|id| id > 0), "an entry names a worklog: {:?}", entry.worklog_id);
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_duration_jira_cannot_parse() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("unparsable duration"))).await;

    let error = cloud()
        .issue_worklogs()
        .add_worklog(&issue.key, worklog_of("not a duration", None))
        .send()
        .await
        .expect_err("a duration Jira cannot parse is not logged");

    assert_eq!(error.status(), Some(400), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_worklogs_of_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_worklogs()
        .get_issue_worklog(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist has no worklogs");

    assert!(error.is_not_found(), "{error}");
}

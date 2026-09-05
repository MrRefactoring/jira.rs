//! The Agile `backlog` API, and the board-side reads that are the only way to see what it did.
//!
//! The backlog is not a container an issue is put into: it is where an issue sits when it belongs to no sprint, so
//! "move to backlog" is really "remove from sprint". The two endpoints differ only in whether ranking is possible —
//! the board-scoped one accepts `rankBeforeIssue`, the global one does not — and both answer 204, which is why what
//! is asserted here is a read taken afterwards rather than anything the write hands back.

use crate::harness::{ResourceTracker, agile, create_test_issue, scrum_board, test_name};

/// The shape of every Jira issue key: a project key, a hyphen, a number.
fn is_issue_key(key: &str) -> bool {
    key.split_once('-').is_some_and(|(project, number)| {
        project.starts_with(|character: char| character.is_ascii_uppercase())
            && project.chars().all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
            && !number.is_empty()
            && number.chars().all(|character| character.is_ascii_digit())
    })
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn moves_an_issue_to_the_backlog_and_reads_the_board_back() {
    let mut tracker = ResourceTracker::new();
    let board_id = scrum_board(&mut tracker).await;

    let issue = create_test_issue(&mut tracker, Some(&test_name("backlog candidate"))).await;

    let backlog =
        agile().board().get_issues_for_backlog(board_id).max_results(10).send().await.expect("the board has a backlog");

    assert!(backlog.issues.len() <= 10, "ten rows were asked for, {} arrived", backlog.issues.len());
    assert!(
        backlog.issues.iter().all(|row| is_issue_key(&row.key)),
        "every row of a backlog is an issue: {:?}",
        backlog.issues.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(),
    );

    // The count is a separate endpoint a site can refuse; what matters is that the refusal is typed rather than a
    // body that fails to parse.
    match agile().board().get_approximate_issue_count_for_backlog(board_id).send().await {
        Ok(count) => assert!(count.count.is_some(), "a count endpoint answers with a count"),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }

    agile()
        .backlog()
        .move_issues_to_backlog([issue.key.as_str()])
        .send()
        .await
        .expect("an issue can be moved to the backlog, and the move answers with nothing");

    let filtered = agile()
        .board()
        .get_issues_for_backlog(board_id)
        .jql(format!("key = {}", issue.key))
        .send()
        .await
        .expect("the backlog listing takes a JQL filter");

    assert!(
        filtered.issues.iter().all(|row| row.key == issue.key),
        "the JQL filter narrows the backlog rather than being ignored: {:?}",
        filtered.issues.iter().map(|row| row.key.as_str()).collect::<Vec<_>>(),
    );

    agile()
        .backlog()
        .move_issues_to_backlog_for_board(board_id, [issue.key.as_str()])
        .send()
        .await
        .expect("the board-scoped variant, the one that could also rank, accepts the same issue");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_issue_key_that_does_not_exist() {
    let error = agile()
        .backlog()
        .move_issues_to_backlog(["NOSUCH-1"])
        .send()
        .await
        .expect_err("an issue that does not exist cannot leave a sprint");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_board_that_does_not_exist() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("backlog board"))).await;

    let error = agile()
        .backlog()
        .move_issues_to_backlog_for_board(99_999_999, [issue.key.as_str()])
        .send()
        .await
        .expect_err("a board that does not exist has no backlog to move an issue into");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");

    tracker.cleanup().await;
}

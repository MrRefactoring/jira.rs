//! The Agile `issue` API: the same issues as the platform API, seen through the Agile lens.
//!
//! `get_issue` here is a *different endpoint* from `issues().get_issue()`, returning the same issue with
//! board-specific fields attached, and a caller who reaches for the wrong one gets something that looks right and is
//! missing what they needed.
//!
//! Ranking is the other half. Rank is a field no ordinary write touches: it is manipulated only through `rank_issues`,
//! relative to another issue, and it is what a board's ordering actually is.

use jira::agile::{GetAllBoardsRequestType, IssueRankRequest};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, agile, cloud, create_test_issue, test_name};

/// The scrum board estimation is read against, where the site has one.
async fn scrum_board() -> Option<i64> {
    let boards = agile()
        .board()
        .get_all_boards()
        .project_key_or_id(TEST_PROJECT_KEY)
        .r#type(GetAllBoardsRequestType::Scrum)
        .max_results(1)
        .send()
        .await
        .expect("the board listing is accepted");

    boards.values.first().and_then(|board| board.id)
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_issue_with_the_agile_fields_the_platform_endpoint_omits() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("agile lens"))).await;

    let read = agile().issue().get_issue(&issue.key).send().await.expect("the issue reads back through the Agile API");

    assert_eq!(read.id, issue.id);
    assert_eq!(read.key, issue.key);

    let fields = read.fields.expect("the Agile endpoint returns the issue's fields");

    assert!(
        fields.keys().any(|field| field.starts_with("customfield_")),
        "the Agile lens carries the board custom fields — sprint, rank, epic — the plain endpoint leaves out",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn agrees_with_the_platform_endpoint_on_the_fields_they_share() {
    let mut tracker = ResourceTracker::new();
    let summary = test_name("two lenses");
    let issue = create_test_issue(&mut tracker, Some(&summary)).await;

    let via_agile = agile().issue().get_issue(&issue.key).send().await.expect("the Agile endpoint answers");
    let via_platform = cloud().issues().get_issue(&issue.key).send().await.expect("the platform endpoint answers");

    assert_eq!(Some(via_agile.id.as_str()), via_platform.id.as_deref(), "two endpoints, one issue");

    let through_agile =
        via_agile.fields.as_ref().and_then(|fields| fields.get("summary")).and_then(|value| value.as_str());
    let through_platform =
        via_platform.fields.as_ref().and_then(|fields| fields.get("summary")).and_then(|value| value.as_str());

    assert_eq!(through_agile, through_platform);
    assert_eq!(through_agile, Some(summary.as_str()), "both lenses show the summary the issue was created with");

    tracker.cleanup().await;
}

/// Ranking, walked as one sequence because every case needs the same pair of issues to rank against each other.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn ranks_issues_relative_to_one_another() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("rank first"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("rank second"))).await;

    agile()
        .issue()
        .rank_issues(IssueRankRequest {
            issues: Some(vec![second.key.clone()]),
            rank_after_issue: Some(first.key.clone()),
            ..IssueRankRequest::default()
        })
        .send()
        .await
        .expect("one issue can be ranked after another");

    let ranked = agile().issue().get_issue(&second.key).send().await.expect("the ranked issue reads back");

    assert_eq!(ranked.key, second.key, "a rank moves the issue in the ordering, not out of the project");

    agile()
        .issue()
        .rank_issues(IssueRankRequest {
            issues: Some(vec![second.key.clone()]),
            rank_before_issue: Some(first.key.clone()),
            ..IssueRankRequest::default()
        })
        .send()
        .await
        .expect("one issue can be ranked before another, and the rank answers with nothing at all");

    // Ranking an issue relative to itself is the edge worth pinning: nothing in the types stops a caller writing it,
    // and Jira accepts it rather than refusing it.
    agile()
        .issue()
        .rank_issues(IssueRankRequest {
            issues: Some(vec![first.key.clone()]),
            rank_after_issue: Some(first.key.clone()),
            ..IssueRankRequest::default()
        })
        .send()
        .await
        .expect("Jira accepts ranking an issue relative to itself");

    let still_there = agile().issue().get_issue(&first.key).send().await.expect("the self-ranked issue reads back");

    assert_eq!(still_there.key, first.key);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_estimation_for_the_board_or_refuses_typed() {
    let Some(board_id) = scrum_board().await else {
        return;
    };

    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("estimation"))).await;

    // A board that estimates by issue count has no estimation field to report, and says so with a 4xx rather than an
    // empty answer. Either outcome is correct; an untyped failure is not.
    match agile().issue().get_issue_estimation_for_board(&issue.key).board_id(board_id).send().await {
        Ok(estimation) => assert!(
            estimation.field_id.is_some_and(|field| !field.is_empty()),
            "an estimation names the field it was read from",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_missing_issue_as_not_found() {
    let error = agile()
        .issue()
        .get_issue(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist cannot be read through the Agile lens either");

    assert!(error.is_not_found(), "{error}");
}

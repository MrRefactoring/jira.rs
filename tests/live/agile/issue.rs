use jira::agile::IssueRankRequest;

use crate::harness::{
    ResourceTracker, TEST_PROJECT_KEY, agile, await_agile_visibility, cloud, create_test_issue, scrum_board, test_name,
};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_issue_with_the_agile_fields_the_platform_endpoint_omits() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("agile lens"))).await;

    await_agile_visibility(&issue.key).await;

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

    await_agile_visibility(&issue.key).await;

    let via_agile = agile().issue().get_issue(&issue.key).send().await.expect("the Agile endpoint answers");
    let via_platform = cloud().issues().get_issue(&issue.key).send().await.expect("the platform endpoint answers");

    assert_eq!(Some(via_agile.id.as_str()), via_platform.id.as_deref(), "two endpoints, one issue");

    let through_agile =
        via_agile.fields.as_ref().and_then(|fields| fields.get("summary")).and_then(|value| value.as_str());
    let through_platform = via_platform.fields.as_ref().and_then(|fields| fields.summary.as_deref());

    assert_eq!(through_agile, through_platform);
    assert_eq!(through_agile, Some(summary.as_str()), "both lenses show the summary the issue was created with");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn ranks_issues_relative_to_one_another() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("rank first"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("rank second"))).await;

    await_agile_visibility(&first.key).await;
    await_agile_visibility(&second.key).await;

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
    let mut tracker = ResourceTracker::new();
    let board_id = scrum_board(&mut tracker).await;

    let issue = create_test_issue(&mut tracker, Some(&test_name("estimation"))).await;

    match agile().issue().get_issue_estimation_for_board(&issue.key).board_id(board_id).send().await {
        Ok(estimation) => assert!(
            estimation.field_id.is_some_and(|field| !field.is_empty()),
            "an estimation names the field it was read from",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}"),
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

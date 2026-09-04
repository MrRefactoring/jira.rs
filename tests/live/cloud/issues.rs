use jira::cloud::{IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};

use crate::harness::{
    ResourceTracker, TEST_PROJECT_KEY, await_readable, cloud, create_test_issue, poll_until, test_name,
};

/// The issue lifecycle, end to end.
///
/// These assert the contract rather than that a call resolves: that what the models declare is what arrives, that a
/// mutation is observable on the next read, that query parameters have an effect, and that a deleted issue surfaces
/// as a typed not-found rather than an untyped failure.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_an_issue_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("lifecycle"))).await;

    assert!(issue.id.chars().all(|c| c.is_ascii_digit()), "an id is digits: {}", issue.id);
    assert!(issue.key.starts_with(&format!("{TEST_PROJECT_KEY}-")), "a key is the project and a number: {}", issue.key,);

    let by_key = cloud().issues().get_issue(&issue.key).send().await.expect("the issue reads back by key");
    let by_id = cloud().issues().get_issue(&issue.id).send().await.expect("the issue reads back by id");

    assert_eq!(by_key.id.as_deref(), Some(issue.id.as_str()));
    assert_eq!(by_id.key.as_deref(), Some(issue.key.as_str()));

    let edited = test_name("edited");

    cloud()
        .issues()
        .edit_issue(
            &issue.key,
            IssueUpdateDetails {
                fields: Some(IssueFields { summary: Some(edited.clone()), ..IssueFields::default() }),
                ..IssueUpdateDetails::default()
            },
        )
        .send()
        .await
        .expect("the summary can be edited");

    let after_edit =
        await_readable("the edited issue reads back", || cloud().issues().get_issue(&issue.key).send()).await;

    assert_eq!(
        after_edit.fields.as_ref().and_then(|fields| fields.summary.as_deref()),
        Some(edited.as_str()),
        "the edit is observable on the next read",
    );

    let trimmed = cloud()
        .issues()
        .get_issue(&issue.key)
        .fields(["summary"])
        .send()
        .await
        .expect("the fields parameter is accepted");

    let fields = trimmed.fields.expect("the field asked for arrives");

    assert_eq!(fields.summary.as_deref(), Some(edited.as_str()), "the field asked for arrives");
    assert!(
        fields.issuetype.is_none() && fields.additional.is_empty(),
        "the fields parameter trims the response: {fields:?}",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_the_issue_through_jql_once_indexing_catches_up() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("searchable"))).await;

    let found = poll_until("the issue to be indexed", || async {
        let page = cloud()
            .issue_search()
            .search_issues()
            .jql(format!("key = {}", issue.key))
            .max_results(1)
            .send()
            .await
            .expect("a JQL search is accepted");

        page.issues.filter(|issues| !issues.is_empty())
    })
    .await;

    assert_eq!(found[0].id.as_deref(), Some(issue.id.as_str()));

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_missing_issue_as_not_found() {
    let error = cloud()
        .issues()
        .get_issue(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
    assert!(!error.is_scope(), "a missing issue is not a missing scope");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_unknown_project_with_a_typed_error_rather_than_a_hang() {
    let error = cloud()
        .issues()
        .create_issue(IssueUpdateDetails {
            fields: Some(IssueFields {
                project: Some(Project { key: Some("NOSUCHPROJ".to_owned()), ..Project::default() }),
                issuetype: Some(IssueTypeDetails { name: Some("Task".to_owned()), ..IssueTypeDetails::default() }),
                summary: Some("x".to_owned()),
                ..IssueFields::default()
            }),
            ..IssueUpdateDetails::default()
        })
        .send()
        .await
        .expect_err("a project that does not exist cannot take an issue");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

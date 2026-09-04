//! Ported from jira.js/tests/live/cloud/issueSearch.test.ts.
//!
//! Search is the one part of Jira that is emphatically *not* read-your-write: an issue exists the moment it is
//! created but reaches the index a moment later. Worse, the visibility is not monotonic — an issue the index has
//! already returned once can briefly disappear from it again under load, which is why every read of the index here
//! polls rather than trusting a single warm-up.
//!
//! The other thing this file pins is the field-selection contract, which surprises people: without an explicit
//! `fields`, search answers with ids and nothing else.

use jira::cloud::{Issue, IssuesAndJQLQueries, JQLCountRequest};
use jira::futures_util::TryStreamExt;
use jira::jql::field;

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, poll_until, test_name};

/// Runs the query until the index has caught up with it, and hands back what it matched.
async fn search(jql: &str, fields: Option<&str>) -> Vec<Issue> {
    poll_until("the issue to be indexed", || async {
        let mut request = cloud().issue_search().search_issues().jql(jql);

        if let Some(field) = fields {
            request = request.fields([field]);
        }

        let page = request.send().await.expect("a JQL search is accepted");

        page.issues.filter(|issues| !issues.is_empty())
    })
    .await
}

/// The field-selection contract, which is the whole reason search results look empty to newcomers.
///
/// Without `fields` the index answers with identifiers alone; with it, the response carries exactly what was asked
/// for and nothing more. The last leg proves the query is a real text search rather than a key lookup.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_ids_alone_until_fields_are_asked_for() {
    let mut tracker = ResourceTracker::new();
    let summary = test_name("searchable");
    let issue = create_test_issue(&mut tracker, Some(&summary)).await;

    let bare = search(&format!("key = {}", issue.key), None).await;

    assert_eq!(bare.len(), 1, "a key lookup matches exactly one issue");
    assert_eq!(bare[0].id.as_deref(), Some(issue.id.as_str()));
    assert!(bare[0].fields.is_none(), "no fields were asked for, so none arrive");

    let selected = search(&format!("key = {}", issue.key), Some("summary")).await;
    let fields = selected[0].fields.as_ref().expect("the fields asked for arrive");

    assert_eq!(fields.summary.as_deref(), Some(summary.as_str()));
    assert!(
        fields.issuetype.is_none() && fields.additional.is_empty(),
        "exactly the field asked for, and no other: {fields:?}",
    );

    let by_text = search(&format!("project = {TEST_PROJECT_KEY} AND summary ~ \"searchable\""), Some("summary")).await;

    assert!(
        by_text.iter().any(|row| row.id.as_deref() == Some(issue.id.as_str())),
        "the issue is findable by text, not only by key",
    );

    tracker.cleanup().await;
}

/// The new search endpoint pages with an opaque token rather than an offset, so a second page is asked for by
/// handing the first one's token back.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_with_a_token_rather_than_an_offset() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("paged search"))).await;

    search(&format!("key = {}", issue.key), None).await;

    let jql = format!("project = {TEST_PROJECT_KEY} ORDER BY created DESC");

    let first_page = cloud()
        .issue_search()
        .search_issues()
        .jql(jql.as_str())
        .max_results(1)
        .send()
        .await
        .expect("the first page is readable");

    assert_eq!(first_page.issues.as_ref().map(Vec::len), Some(1), "the limit is honoured, not merely echoed");

    if let Some(token) = first_page.next_page_token.clone() {
        let second_page = cloud()
            .issue_search()
            .search_issues()
            .jql(jql.as_str())
            .max_results(1)
            .next_page_token(token)
            .send()
            .await
            .expect("the token names a further page");

        let first = first_page.issues.as_ref().and_then(|issues| issues[0].id.clone());
        let second = second_page.issues.as_ref().and_then(|issues| issues[0].id.clone());

        assert_ne!(second, first, "the token moves the window rather than repeating it");
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn counts_matches_without_returning_them() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("counted"))).await;

    let count = poll_until("the count to see the issue", || async {
        let result = cloud()
            .issue_search()
            .count_issues(JQLCountRequest { jql: Some(format!("key = {}", issue.key)) })
            .send()
            .await
            .expect("an approximate count is accepted");

        result.count.filter(|count| *count == 1)
    })
    .await;

    assert_eq!(count, 1, "the key matches one issue and the count says so without returning it");

    tracker.cleanup().await;
}

/// `jql/match` answers per query rather than per issue, and a query that matches nothing is an empty list rather
/// than an error entry.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn tests_issues_against_queries_without_running_a_search() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("matched"))).await;
    let id: i64 = issue.id.parse().expect("an issue id is a number");

    let result = cloud()
        .issue_search()
        .match_issues(IssuesAndJQLQueries {
            issue_ids: vec![id],
            jqls: vec![format!("project = {TEST_PROJECT_KEY}"), "project = NOSUCHPROJECT".to_owned()],
        })
        .send()
        .await
        .expect("issues can be matched against queries");

    assert_eq!(result.matches.len(), 2, "one answer per query, in the order they were given");
    assert!(result.matches[0].matched_issues.contains(&id), "the issue is in its own project");
    assert!(result.matches[1].matched_issues.is_empty(), "a project that does not exist matches nothing");
    assert!(result.matches[1].errors.is_empty(), "matching nothing is not an error");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn suggests_issues_through_the_picker() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("picked"))).await;

    let picker = cloud()
        .issue_search()
        .get_issue_picker_resource()
        .query(&issue.key)
        .send()
        .await
        .expect("the picker answers a query");

    let sections = picker.sections.expect("the picker answers in sections");

    assert!(!sections.is_empty(), "the picker offers at least one section");
    assert!(
        sections.iter().all(|section| section.label.as_deref().is_some_and(|label| !label.is_empty())),
        "every section is labelled for a user to read",
    );

    tracker.cleanup().await;
}

/// Jira treats a string that is not JQL as free text rather than rejecting it, which is why a typo in a query
/// silently returns nothing instead of failing.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn accepts_bare_words_as_a_text_search_rather_than_rejecting_them() {
    let page = cloud()
        .issue_search()
        .search_issues()
        .jql("this is not jql")
        .send()
        .await
        .expect("bare words are a text search, not a syntax error");

    assert!(page.issues.unwrap_or_default().is_empty(), "the words match no issue");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_genuinely_malformed_jql_with_a_typed_400() {
    let error = cloud()
        .issue_search()
        .search_issues()
        .jql("project = \"unterminated")
        .send()
        .await
        .expect_err("an unterminated string is not a query");

    assert_eq!(error.status(), Some(400), "{error}");
    assert!(!error.is_not_found(), "malformed input is not a missing resource");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_an_unmatched_query_with_an_empty_result_rather_than_an_error() {
    let page = cloud()
        .issue_search()
        .search_issues()
        .jql(format!("project = {TEST_PROJECT_KEY} AND summary ~ \"nothingmatchesthisatall\""))
        .send()
        .await
        .expect("a query matching nothing is still a valid query");

    assert!(page.issues.unwrap_or_default().is_empty(), "nothing matched, so nothing is returned");
}

/// The builder puts a value into a query without letting it become part of the query.
///
/// A summary carrying a quotation mark is the case a `format!`-built query cannot survive: the mark closes the
/// literal early and Jira answers 400 for a query it cannot parse. What is asserted here is that the request is
/// accepted at all — the escaping is the subject, and the site is the only thing that can judge it.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn a_quotation_mark_in_a_value_reaches_jira_as_a_value() {
    let mut tracker = ResourceTracker::new();
    let summary = format!(r#"{} say "hello""#, test_name("quoted"));
    let issue = create_test_issue(&mut tracker, Some(&summary)).await;

    let by_key = cloud()
        .issue_search()
        .search_issues()
        .jql(field("key").eq(issue.key.as_str()).and(field("summary").contains(&summary)))
        .fields(["summary"])
        .send()
        .await
        .expect("a query carrying a quotation mark is parsed rather than rejected");

    assert!(by_key.issues.is_some(), "the query ran, whether or not the index has caught up");

    tracker.cleanup().await;
}

/// The token loop, walked by the crate rather than by the caller.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn a_stream_walks_every_page_of_a_search() {
    let mut tracker = ResourceTracker::new();
    let first = create_test_issue(&mut tracker, Some(&test_name("streamed one"))).await;
    let second = create_test_issue(&mut tracker, Some(&test_name("streamed two"))).await;

    search(&format!("key = {}", first.key), None).await;
    search(&format!("key = {}", second.key), None).await;

    let query = field("key").is_in([first.key.as_str(), second.key.as_str()]).order_by("key");
    let mut issues = cloud()
        .issue_search()
        .search_issues()
        .jql(query)
        // One issue per page, so a stream that stops at the first page cannot pass this.
        .max_results(1)
        .fields(["summary"])
        .stream();
    let mut keys = Vec::new();

    while let Some(issue) = issues.try_next().await.expect("every page of the stream is readable") {
        keys.push(issue.key.expect("a searched issue carries its key"));
    }

    keys.sort();

    let mut expected = vec![first.key.clone(), second.key.clone()];
    expected.sort();

    assert_eq!(keys, expected, "the stream ends at the last page rather than at the first");

    tracker.cleanup().await;
}

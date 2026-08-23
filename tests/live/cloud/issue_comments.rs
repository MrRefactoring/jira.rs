//! Ported from jira.js/tests/live/cloud/issueComments.test.ts.
//!
//! A full create-read-update-delete cycle on comments attached to fixture issues — safe, self-cleaning, and the
//! densest write surface the library has.
//!
//! The rich-text routing these endpoints perform is covered separately in `adf_routing`. Here the concern is
//! everything else: paging, ordering, `expand`, and that an update is a replacement.

use jira::cloud::{
    Comment, CommentInput, CommentInputBody, DocumentType, GetCommentRequestExpand, GetCommentRequestExpandVariant2,
    GetCommentsRequestOrderBy, IssueCommentListRequest, PageOfComments,
};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, document_of, test_name};

fn comment_of(text: &str) -> CommentInput {
    CommentInput {
        body: Some(CommentInputBody::Document(document_of(text))),
        ..CommentInput::default()
    }
}

fn body_text(comment: &Comment) -> String {
    serde_json::to_string(&comment.body).expect("a comment body is serialisable")
}

fn comment_ids(page: &PageOfComments) -> Vec<String> {
    page.comments
        .iter()
        .flatten()
        .filter_map(|comment| comment.id.clone())
        .collect()
}

/// Adds a comment to the issue and registers its deletion.
async fn add_comment(tracker: &mut ResourceTracker, issue_key: &str, text: &str) -> Comment {
    let created = cloud()
        .issue_comments()
        .add_comment(issue_key, comment_of(text))
        .send()
        .await
        .expect("the issue takes a comment");

    let key = issue_key.to_owned();
    let id = created.id.clone().expect("a created comment carries an id");

    tracker.defer(move || {
        let (key, id) = (key.clone(), id.clone());

        async move { cloud().issue_comments().delete_comment(key, id).send().await }
    });

    created
}

/// The comment lifecycle, end to end.
///
/// Proves the shape creation hands back, that a read by id matches it, that an update replaces the body rather than
/// appending to it, and that a deleted comment is gone from both the direct read and the listing.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_comment_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("comments"))).await;

    let empty = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .send()
        .await
        .expect("a fresh issue lists comments");

    assert_eq!(empty.total, Some(0));
    assert!(
        empty.comments.is_none_or(|comments| comments.is_empty()),
        "a fresh issue carries no comments"
    );

    let created = add_comment(&mut tracker, &issue.key, "first comment").await;
    let comment_id = created.id.clone().expect("a created comment carries an id");

    assert!(
        comment_id.chars().all(|character| character.is_ascii_digit()),
        "an id is digits: {comment_id}"
    );
    assert!(
        created.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a comment carries its own URL: {:?}",
        created.self_,
    );
    assert!(
        created
            .author
            .as_ref()
            .and_then(|author| author.account_id.as_deref())
            .is_some_and(|id| !id.is_empty()),
        "a comment carries the account that wrote it",
    );

    let body = created.body.as_ref().expect("a comment carries a document body");

    assert_eq!(body.r#type, DocumentType::Doc);
    assert!(
        (body.version - 1.0).abs() < f64::EPSILON,
        "the body is version 1 ADF, got {}",
        body.version
    );

    let created_at = created.created.clone().expect("a comment carries a creation timestamp");

    assert!(
        created_at.contains('T'),
        "a timestamp is an ISO 8601 instant: {created_at}"
    );
    assert_eq!(
        created.updated.as_deref(),
        Some(created_at.as_str()),
        "a fresh comment was never updated"
    );

    let fetched = cloud()
        .issue_comments()
        .get_comment(&issue.key, &comment_id)
        .send()
        .await
        .expect("the comment reads back by id");

    assert_eq!(fetched.id.as_deref(), Some(comment_id.as_str()));
    assert!(body_text(&fetched).contains("first comment"), "{}", body_text(&fetched));

    let edited = cloud()
        .issue_comments()
        .update_comment(&issue.key, &comment_id, comment_of("edited comment"))
        .send()
        .await
        .expect("the comment body can be replaced");

    assert!(body_text(&edited).contains("edited comment"), "{}", body_text(&edited));
    assert!(
        !body_text(&edited).contains("first comment"),
        "an update replaces the body rather than appending to it"
    );
    // Timestamps within one response carry the same UTC offset, so lexicographic order is chronological order.
    assert!(
        edited.updated.as_deref() > Some(created_at.as_str()),
        "the edit moves `updated` past `created`"
    );

    cloud()
        .issue_comments()
        .delete_comment(&issue.key, &comment_id)
        .send()
        .await
        .expect("the comment can be deleted");

    let error = cloud()
        .issue_comments()
        .get_comment(&issue.key, &comment_id)
        .send()
        .await
        .expect_err("a deleted comment cannot be read");

    assert!(error.is_not_found(), "{error}");

    let remaining = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .send()
        .await
        .expect("the listing reads after a delete");

    assert_eq!(remaining.total, Some(0));
    assert!(
        !comment_ids(&remaining).contains(&comment_id),
        "the deleted comment is gone from the listing"
    );

    tracker.cleanup().await;
}

/// `renderedBody` costs the server a rendering pass, so it arrives only when `expand` asks for it.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn renders_the_body_as_html_only_when_expand_asks() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("rendered comment"))).await;
    let created = add_comment(&mut tracker, &issue.key, "first comment").await;
    let comment_id = created.id.clone().expect("a created comment carries an id");

    let plain = cloud()
        .issue_comments()
        .get_comment(&issue.key, &comment_id)
        .send()
        .await
        .expect("the comment reads back unexpanded");

    let rendered = cloud()
        .issue_comments()
        .get_comment(&issue.key, &comment_id)
        .expand(GetCommentRequestExpand::Variant2(
            GetCommentRequestExpandVariant2::RenderedBody,
        ))
        .send()
        .await
        .expect("`expand=renderedBody` is accepted");

    assert!(
        plain.rendered_body.is_none(),
        "the body is not rendered unless asked for"
    );

    let html = rendered.rendered_body.expect("`expand=renderedBody` renders the body");

    assert!(html.contains("first comment"), "{html}");
    assert!(html.contains("<p>"), "the rendered body is HTML: {html}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_and_orders_the_comment_list() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("paged comments"))).await;

    for text in ["first", "second", "third"] {
        add_comment(&mut tracker, &issue.key, text).await;
    }

    let all = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .send()
        .await
        .expect("the comments list");

    assert_eq!(all.total, Some(3));

    let first_page = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .max_results(2)
        .send()
        .await
        .expect("`maxResults` is accepted");

    assert_eq!(first_page.comments.as_ref().map(Vec::len), Some(2));
    assert_eq!(first_page.max_results, Some(2));

    let second_page = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .start_at(2)
        .send()
        .await
        .expect("`startAt` is accepted");

    assert_eq!(second_page.start_at, Some(2));
    assert_eq!(second_page.comments.as_ref().map(Vec::len), Some(1));

    let descending = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .order_by(GetCommentsRequestOrderBy::CreatedDescending)
        .send()
        .await
        .expect("`orderBy` is accepted");

    let mut reversed = comment_ids(&all);
    reversed.reverse();

    assert_eq!(
        comment_ids(&descending),
        reversed,
        "`-created` is the default listing reversed"
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fetches_comments_by_id_in_one_call() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("comments by id"))).await;

    for text in ["first", "second"] {
        add_comment(&mut tracker, &issue.key, text).await;
    }

    let listing = cloud()
        .issue_comments()
        .get_comments(&issue.key)
        .send()
        .await
        .expect("the comments list");
    let mut expected = comment_ids(&listing);
    let ids = expected
        .iter()
        .map(|id| id.parse().expect("a comment id is a number"))
        .collect();

    let page = cloud()
        .issue_comments()
        .get_comments_by_ids(IssueCommentListRequest { ids })
        .send()
        .await
        .expect("comments can be fetched by id across issues");

    let mut returned: Vec<String> = page.values.iter().filter_map(|comment| comment.id.clone()).collect();

    expected.sort();
    returned.sort();

    assert_eq!(returned, expected);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_comments_of_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_comments()
        .get_comments(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist has no comments");

    assert!(error.is_not_found(), "{error}");
}

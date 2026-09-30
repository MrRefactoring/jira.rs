use jira::cloud::{
    Comment, CommentInput, CommentInputBody, Document, IssueFields, IssueFieldsDescription, IssueUpdateDetails,
    Worklog, WorklogInput, WorklogInputComment,
};
use serde_json::Value;

use crate::harness::{
    ResourceTracker, await_readable, client, cloud, create_issue_with, create_test_issue, document_of, poll_until,
    test_issue_fields, test_name,
};

fn node_types(document: &Document) -> Vec<String> {
    let value = serde_json::to_value(document).expect("a document is serialisable");
    let mut types = Vec::new();

    collect_types(&value, &mut types);

    types
}

fn collect_types(node: &Value, types: &mut Vec<String>) {
    if let Some(kind) = node.get("type").and_then(Value::as_str) {
        types.push(kind.to_owned());
    }

    for child in node.get("content").and_then(Value::as_array).into_iter().flatten() {
        collect_types(child, types);
    }
}

fn comment_of(text: &str) -> CommentInput {
    CommentInput { body: Some(CommentInputBody::Document(document_of(text))), ..CommentInput::default() }
}

fn rendered(document: &Document) -> String {
    serde_json::to_string(document).expect("a document is serialisable")
}

async fn add_comment(tracker: &mut ResourceTracker, issue_key: &str, text: &str) -> Comment {
    let created = cloud()
        .issue_comments()
        .add_comment(issue_key, comment_of(text))
        .send()
        .await
        .expect("the issue takes a document as a comment");

    let key = issue_key.to_owned();
    let id = created.id.clone().expect("a created comment carries an id");

    tracker.defer(move || {
        let (key, id) = (key.clone(), id.clone());

        async move { cloud().issue_comments().delete_comment(key, id).send().await }
    });

    let readable = created.id.clone().expect("a created comment carries an id");

    poll_until("the comment just added to read back", || async {
        cloud().issue_comments().get_comment(issue_key, &readable).send().await.ok()
    })
    .await;

    created
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn sends_a_document_to_v3_and_reads_the_same_document_back() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("adf routing"))).await;
    let created = add_comment(&mut tracker, &issue.key, "untouched").await;
    let comment_id = created.id.clone().expect("a created comment carries an id");
    let body = created.body.as_ref().expect("a comment comes back with a document body");

    assert_eq!(node_types(body), ["doc", "paragraph", "text"], "Jira added nothing to the document and lost nothing");
    assert!(rendered(body).contains("untouched"), "the text arrives verbatim: {}", rendered(body));

    let fetched = await_readable("the comment reads back by id", || {
        cloud().issue_comments().get_comment(&issue.key, &comment_id).send()
    })
    .await;

    let stored = fetched.body.as_ref().expect("a stored comment carries a document body");

    assert_eq!(
        serde_json::to_value(stored).expect("a document is serialisable"),
        serde_json::to_value(body).expect("a document is serialisable"),
        "a request later the document is the one that was sent",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn leaves_the_stored_document_readable_through_v2_as_markup() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("adf through v2"))).await;
    let created = add_comment(&mut tracker, &issue.key, "still markup").await;
    let comment_id = created.id.clone().expect("a created comment carries an id");

    let raw = client()
        .get(format!("/rest/api/2/issue/{}/comment/{comment_id}", issue.key))
        .send_raw()
        .await
        .expect("the v2 endpoint serves the comment v3 created");

    let body = raw.get("body").and_then(Value::as_str).expect("v2 renders the body as a string rather than an object");

    assert!(body.contains("still markup"), "the stored text survives the conversion to markup: {body}");
    assert_eq!(raw.get("id").and_then(Value::as_str), Some(comment_id.as_str()), "v2 answers about the same comment");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn routes_a_worklog_comment_the_same_way() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("adf worklog"))).await;

    let worklog: Worklog = cloud()
        .issue_worklogs()
        .add_worklog(
            &issue.key,
            WorklogInput {
                comment: Some(WorklogInputComment::Document(document_of("worklog note"))),
                time_spent: Some("5m".to_owned()),
                ..WorklogInput::default()
            },
        )
        .send()
        .await
        .expect("the issue takes a worklog carrying a document");

    let key = issue.key.clone();
    let worklog_id = worklog.id.clone().expect("a created worklog carries an id");
    let deferred = worklog_id.clone();

    tracker.defer(move || {
        let (key, id) = (key.clone(), deferred.clone());

        async move { cloud().issue_worklogs().delete_worklog(key, id).send().await }
    });

    let comment = worklog.comment.as_ref().expect("a worklog comes back with a document comment");

    assert_eq!(node_types(comment), ["doc", "paragraph", "text"], "a worklog comment is a document like any other");
    assert!(rendered(comment).contains("worklog note"), "{}", rendered(comment));
    assert_eq!(worklog.time_spent.as_deref(), Some("5m"), "the worklog kept the time it was given");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn accepts_a_document_as_a_description_at_issue_creation() {
    let mut tracker = ResourceTracker::new();

    let created = create_issue_with(
        &mut tracker,
        IssueFields {
            description: Some(IssueFieldsDescription::Document(document_of("described in a document"))),
            ..test_issue_fields(test_name("described"))
        },
    )
    .await;

    let fetched = cloud().issues().get_issue(&created.key).send().await.expect("the described issue reads back");

    let description = fetched
        .fields
        .and_then(|fields| fields.description)
        .expect("the issue carries the description it was created with");
    let IssueFieldsDescription::Document(document) = description else {
        panic!("stored as a document: {description:?}");
    };
    let description = serde_json::to_value(document).expect("a document serializes");

    let mut types = Vec::new();

    collect_types(&description, &mut types);

    assert!(types.contains(&"paragraph".to_owned()), "the paragraph survived creation: {types:?}");
    assert!(description.to_string().contains("described in a document"), "{description}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn converts_wiki_markup_written_as_a_description_at_edit_into_a_document() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("edited"))).await;

    cloud()
        .issues()
        .edit_issue(
            &issue.key,
            IssueUpdateDetails {
                fields: Some(IssueFields {
                    description: Some(IssueFieldsDescription::Variant1(
                        "h2. Heading\n\n*bold* and _italic_".to_owned(),
                    )),
                    ..IssueFields::default()
                }),
                ..IssueUpdateDetails::default()
            },
        )
        .send()
        .await
        .expect("a description written as wiki markup is accepted at edit");

    let fetched = cloud().issues().get_issue(&issue.key).send().await.expect("the edited issue reads back");

    let description = fetched
        .fields
        .and_then(|fields| fields.description)
        .expect("the issue carries the description it was edited to");
    let IssueFieldsDescription::Document(document) = description else {
        panic!("stored as a document: {description:?}");
    };
    let rendered = serde_json::to_string(&document).expect("a document serializes");

    assert!(rendered.contains("\"heading\""), "`h2.` became a heading: {rendered}");
    assert!(rendered.contains("\"strong\""), "`*bold*` became a strong mark: {rendered}");
    assert!(rendered.contains("\"em\""), "`_italic_` became an emphasis mark: {rendered}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn converts_wiki_markup_written_as_a_string_into_a_document() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("markup"))).await;

    let comment = cloud()
        .issue_comments()
        .add_comment(
            &issue.key,
            CommentInput {
                body: Some(CommentInputBody::Variant1("h2. Heading\n\n*bold* and _italic_".to_owned())),
                ..CommentInput::default()
            },
        )
        .send()
        .await
        .expect("a comment written as wiki markup is accepted");

    let document = comment.body.expect("a comment written through v2 reads back as a document");

    let rendered = serde_json::to_string(&document).expect("a document serializes");

    assert!(rendered.contains("\"heading\""), "`h2.` became a heading: {rendered}");
    assert!(rendered.contains("\"strong\""), "`*bold*` became a strong mark: {rendered}");
    assert!(rendered.contains("\"em\""), "`_italic_` became an emphasis mark: {rendered}");

    tracker.cleanup().await;
}

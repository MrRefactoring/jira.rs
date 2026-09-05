use jira::cloud::{CreatedIssue, Document, IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};
use serde_json::json;

use super::client::{agile, cloud};
use super::naming::test_name;
use super::poll::poll_until;
use super::resources::ResourceTracker;

/// The project every Cloud suite works in. Its issue types are `Task` and `Sub-task`.
///
/// Issues are created in an existing project rather than a fresh one: creating a Jira project is slow, consumes a
/// licence slot, and often cannot be deleted cleanly by the same token that made it. A dedicated test project is the
/// cheaper and more reliable unit of isolation, and run-scoped names keep concurrent runs apart inside it.
pub const TEST_PROJECT_KEY: &str = "AUTOTEST";

/// The issue type used unless a suite needs something else.
pub const TEST_ISSUE_TYPE: &str = "Task";

/// A minimal ADF document wrapping one line of text.
pub fn document_of(text: &str) -> Document {
    serde_json::from_value(json!({
        "type": "doc",
        "version": 1,
        "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": text }] }],
    }))
    .expect("a hand-built ADF paragraph is a document")
}

/// The fields every issue the suites create starts from: the test project, the default type and a summary.
pub fn test_issue_fields(summary: String) -> IssueFields {
    IssueFields {
        project: Some(Project { key: Some(TEST_PROJECT_KEY.to_owned()), ..Project::default() }),
        issuetype: Some(IssueTypeDetails { name: Some(TEST_ISSUE_TYPE.to_owned()), ..IssueTypeDetails::default() }),
        summary: Some(summary),
        ..IssueFields::default()
    }
}

/// Creates an issue in the test project and registers its deletion.
pub async fn create_test_issue(tracker: &mut ResourceTracker, summary: Option<&str>) -> CreatedIssue {
    create_issue_with(tracker, test_issue_fields(summary.map_or_else(|| test_name("issue"), ToOwned::to_owned))).await
}

/// Creates an issue from the fields given, registers its deletion, and waits for it to be readable.
///
/// A key that `create_issue` has just answered with is not yet an issue every endpoint can see: for a second or so
/// after the write, `getIssue`, the worklog endpoints and the watcher endpoints all answer 404 with "Issue does not
/// exist or you do not have permission to see it". Waiting here rather than in each caller is what keeps the whole
/// class fixed instead of the three cases that happened to fail on the day someone looked.
pub async fn create_issue_with(tracker: &mut ResourceTracker, fields: IssueFields) -> CreatedIssue {
    let created = cloud()
        .issues()
        .create_issue(IssueUpdateDetails { fields: Some(fields), ..IssueUpdateDetails::default() })
        .send()
        .await
        .expect("the test project accepts a new issue");

    let key = created.key.clone();

    tracker.defer(move || {
        let key = key.clone();

        async move { cloud().issues().delete_issue(key).send().await }
    });

    poll_until("the issue just created to read back", || async {
        cloud().issues().get_issue(&created.key).send().await.ok()
    })
    .await;

    created
}

/// Waits until the Agile lens can see an issue the platform API has already created.
///
/// The Agile endpoints read their own index, which catches up on Jira's schedule rather than on the write's. An issue
/// that `create_test_issue` has just returned a key for is answered with a 404 by `agile().issue()` and with a 400 by
/// `rank_issues` until that index has it, and both refusals say "Issue does not exist or you do not have permission to
/// see it" — which is indistinguishable from the real thing at the call site.
pub async fn await_agile_visibility(key: &str) {
    poll_until("the Agile index to see the issue", || async { agile().issue().get_issue(key).send().await.ok() }).await;
}

/// A scrum board over the test project, and the filter it is built on.
#[derive(Debug, Clone, Copy)]
pub struct TestBoard {
    pub id: i64,
    pub filter_id: i64,
}

/// Creates a scrum board over the test project and registers the removal of both it and its filter.
///
/// A board needs a filter, and Jira makes the filter visible to the board service a moment after it is created, so
/// the creation is retried while it says the filter is not available yet. Without that the first attempt fails on a
/// site that is otherwise perfectly configured.
pub async fn create_test_board(tracker: &mut ResourceTracker) -> TestBoard {
    let filter = cloud()
        .filters()
        .create_filter(jira::cloud::Filter {
            name: test_name("board filter"),
            jql: Some(format!("project = {TEST_PROJECT_KEY} ORDER BY Rank ASC")),
            ..jira::cloud::Filter::default()
        })
        .send()
        .await
        .expect("the account may create a filter of its own");

    let filter_id: i64 =
        filter.id.as_deref().expect("a created filter has an id").parse().expect("a filter id is a number");

    tracker.defer(move || async move { cloud().filters().delete_filter(filter_id).send().await });

    let name: String = test_name("board").chars().take(40).collect();
    let mut board = None;
    let mut delay = std::time::Duration::from_millis(500);

    for attempt in 0..6 {
        let request = jira::agile::BoardCreate {
            name: Some(name.clone()),
            r#type: Some(jira::agile::BoardCreateType::Scrum),
            filter_id: Some(filter_id),
            location: Some(jira::agile::Location {
                r#type: Some("project".into()),
                project_key_or_id: Some(TEST_PROJECT_KEY.to_owned()),
            }),
        };

        match agile().board().create_board(request).send().await {
            Ok(created) => {
                board = Some(created);
                break;
            }
            Err(error) => {
                let filter_not_visible_yet = error
                    .body()
                    .is_some_and(|body| body.to_string().to_lowercase().contains("filter is not available"));

                assert!(filter_not_visible_yet && attempt < 5, "a scrum board could not be created: {error}");

                tokio::time::sleep(delay).await;
                delay = delay.mul_f64(1.8);
            }
        }
    }

    let id = board.and_then(|created| created.id).expect("a created board carries an id");

    tracker.defer(move || async move { agile().board().delete_board(id).send().await });

    poll_until("the board just created to be servable by the Agile API", || async {
        agile().board().get_board(id).send().await.ok()
    })
    .await;

    TestBoard { id, filter_id }
}

/// The scrum board the Agile suites run against: an existing one where the site has it, a fresh one otherwise.
pub async fn scrum_board(tracker: &mut ResourceTracker) -> i64 {
    let boards = agile()
        .board()
        .get_all_boards()
        .project_key_or_id(TEST_PROJECT_KEY)
        .r#type("scrum")
        .max_results(1)
        .send()
        .await
        .expect("the board listing is accepted");

    let id = match boards.values.first().and_then(|board| board.id) {
        Some(id) => id,
        None => return create_test_board(tracker).await.id,
    };

    poll_until("the board the listing named to be servable by the Agile API", || async {
        agile().board().get_board(id).send().await.ok()
    })
    .await;

    id
}

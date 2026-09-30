use jira::cloud::{CreatedIssue, Document, IssueFields, IssueTypeDetails, IssueUpdateDetails, Project};
use serde_json::json;

use super::client::{agile, cloud};
use super::naming::test_name;
use super::poll::poll_until;
use super::resources::ResourceTracker;

pub const TEST_PROJECT_KEY: &str = "AUTOTEST";

pub const TEST_ISSUE_TYPE: &str = "Task";

pub fn document_of(text: &str) -> Document {
    serde_json::from_value(json!({
        "type": "doc",
        "version": 1,
        "content": [{ "type": "paragraph", "content": [{ "type": "text", "text": text }] }],
    }))
    .expect("a hand-built ADF paragraph is a document")
}

pub fn test_issue_fields(summary: String) -> IssueFields {
    IssueFields {
        project: Some(Project { key: Some(TEST_PROJECT_KEY.to_owned()), ..Project::default() }),
        issuetype: Some(IssueTypeDetails { name: Some(TEST_ISSUE_TYPE.to_owned()), ..IssueTypeDetails::default() }),
        summary: Some(summary),
        ..IssueFields::default()
    }
}

pub async fn create_test_issue(tracker: &mut ResourceTracker, summary: Option<&str>) -> CreatedIssue {
    create_issue_with(tracker, test_issue_fields(summary.map_or_else(|| test_name("issue"), ToOwned::to_owned))).await
}

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

pub async fn await_agile_visibility(key: &str) {
    poll_until("the Agile index to see the issue", || async { agile().issue().get_issue(key).send().await.ok() }).await;
}

#[derive(Debug, Clone, Copy)]
pub struct TestBoard {
    pub id: i64,
    pub filter_id: i64,
}

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

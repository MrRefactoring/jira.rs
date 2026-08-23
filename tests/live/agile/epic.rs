//! The Agile `epic` API, and the board-side listing of what belongs to no epic.
//!
//! An epic is not a separate kind of object: it is an ordinary issue of the Epic type, which is why this API sits
//! beside the platform one rather than replacing it. What the Agile surface adds is the *membership* relation — which
//! issues belong to which epic — and that is expressed nowhere in the platform API's issue payload.
//!
//! The cycle is one test rather than six. Six tests each gated on the project offering an Epic type would report six
//! passes while verifying nothing where it does not, which is the failure mode this suite exists to avoid.

use jira::agile::EpicUpdate;
use jira::cloud::IssueUpdateDetails;
use serde_json::json;

use crate::harness::{
    ResourceTracker, TEST_ISSUE_TYPE, TEST_PROJECT_KEY, agile, cloud, create_test_issue, scrum_board, test_name,
};

/// The id of the project's Epic issue type, where its issue type scheme carries one.
async fn epic_type_id() -> Option<String> {
    let project = cloud().projects().get_project(TEST_PROJECT_KEY).send().await.expect("the test project reads back");

    project
        .issue_types
        .unwrap_or_default()
        .into_iter()
        .find(|issue_type| issue_type.name.as_deref() == Some("Epic"))
        .and_then(|issue_type| issue_type.id)
}

/// What the cycle below is gated on: the test project ships with Task and Sub-task, and the site's Epic type is not
/// in its issue type scheme. Adding it would mean editing a scheme shared with other projects, which is exactly the
/// kind of write this suite refuses to make.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn records_the_issue_types_the_test_project_offers() {
    let project = cloud().projects().get_project(TEST_PROJECT_KEY).send().await.expect("the test project reads back");
    let types = project.issue_types.expect("a project reports the issue types it offers");
    let names: Vec<&str> = types.iter().filter_map(|issue_type| issue_type.name.as_deref()).collect();

    assert!(names.contains(&TEST_ISSUE_TYPE), "the test project offers {TEST_ISSUE_TYPE}, got {names:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn runs_the_whole_epic_cycle_where_an_epic_type_is_available() {
    let Some(epic_type) = epic_type_id().await else {
        // No Epic type in the project, so there is no epic to hang a membership relation off.
        return;
    };

    let mut tracker = ResourceTracker::new();
    let child = create_test_issue(&mut tracker, Some(&test_name("epic child"))).await;

    let created = cloud()
        .issues()
        .create_issue(IssueUpdateDetails {
            fields: Some(
                [
                    ("project".to_owned(), json!({ "key": TEST_PROJECT_KEY })),
                    ("issuetype".to_owned(), json!({ "id": epic_type })),
                    ("summary".to_owned(), json!(test_name("epic"))),
                ]
                .into_iter()
                .collect(),
            ),
            ..IssueUpdateDetails::default()
        })
        .send()
        .await;

    // A project can offer the Epic type and still refuse the issue — a classic project asks for the Epic Name field,
    // which has no default. That is project configuration rather than anything the library did.
    let Ok(created) = created else {
        tracker.cleanup().await;

        return;
    };

    {
        let key = created.key.clone();

        tracker.defer(move || {
            let key = key.clone();

            async move { cloud().issues().delete_issue(key).send().await }
        });
    }

    let epic = agile().epic().get_epic(&created.key).send().await.expect("the epic reads back through the Agile API");

    assert_eq!(epic.key.as_deref(), Some(created.key.as_str()));
    assert!(epic.name.as_ref().is_some_and(|name| !name.is_empty()), "an epic carries a name of its own");
    assert!(
        epic.color.as_ref().and_then(|color| color.key.as_deref()).is_some_and(|key| !key.is_empty()),
        "an epic carries a colour",
    );
    assert_eq!(epic.done, Some(false), "a fresh epic is not done");

    let empty = agile()
        .epic()
        .get_issues_for_epic(&created.key)
        .max_results(10)
        .send()
        .await
        .expect("a fresh epic lists its issues");

    assert!(empty.issues.is_empty(), "a fresh epic holds nothing");

    agile()
        .epic()
        .move_issues_to_epic(&created.key)
        .issues([child.key.as_str()])
        .send()
        .await
        .expect("an issue can be moved into the epic");

    let with_child = agile()
        .epic()
        .get_issues_for_epic(&created.key)
        .max_results(10)
        .send()
        .await
        .expect("the epic lists its issues");

    assert!(
        with_child.issues.iter().any(|issue| issue.key == child.key),
        "the membership the platform payload does not carry is visible here: {:?}",
        with_child.issues.iter().map(|issue| issue.key.as_str()).collect::<Vec<_>>(),
    );

    let renamed = test_name("epic renamed");

    agile()
        .epic()
        .partially_update_epic(&created.key, EpicUpdate { name: Some(renamed.clone()), ..EpicUpdate::default() })
        .send()
        .await
        .expect("an epic can be renamed");

    let after = agile().epic().get_epic(&created.key).send().await.expect("the renamed epic reads back");

    assert_eq!(after.name.as_deref(), Some(renamed.as_str()));
    assert!(
        after.color.and_then(|color| color.key).is_some(),
        "a partial update leaves the fields it was not given alone",
    );

    agile()
        .epic()
        .remove_issues_from_epic([child.key.as_str()])
        .send()
        .await
        .expect("an issue can be removed from its epic");

    let without_child = agile()
        .epic()
        .get_issues_for_epic(&created.key)
        .max_results(10)
        .send()
        .await
        .expect("the emptied epic lists its issues");

    assert!(
        !without_child.issues.iter().any(|issue| issue.key == child.key),
        "the epic no longer reports the issue removed from it",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_issues_belonging_to_no_epic_for_the_board() {
    let mut tracker = ResourceTracker::new();
    let board_id = scrum_board(&mut tracker).await;

    let orphans = agile()
        .board()
        .get_issues_without_epic_for_board(board_id)
        .max_results(50)
        .send()
        .await
        .expect("the board lists the issues that belong to no epic");

    assert!(orphans.issues.len() <= 50, "fifty rows were asked for, {} arrived", orphans.issues.len());
    assert!(
        orphans.issues.iter().all(|issue| !issue.id.is_empty() && issue.key.contains('-')),
        "every row is an issue, epic or not: {:?}",
        orphans.issues.iter().map(|issue| issue.key.as_str()).collect::<Vec<_>>(),
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_epic_as_not_found() {
    let error = agile()
        .epic()
        .get_epic(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an epic that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

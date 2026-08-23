//! Boards, sprints, epics and the backlog, driven against a real Data Center instance.
//!
//! Unlike Cloud, Data Center publishes its agile endpoints in the same document as the platform ones, so they are
//! part of the same client — and they are the half most likely to be missing from a self-hosted deployment, since a
//! Jira without the Software application refuses all of them outright. Reaching them at all is part of what this
//! proves.
//!
//! Every board here comes from a Scrum project the test created, which is the only way to have one on a bare
//! instance; the template makes the board a moment after the project, which is what `board_of` waits for. A test
//! that needs a board covers a whole sequence, because paying for a project and a board once per assertion would
//! spend minutes proving nothing extra.

use jira::server::{
    BoardCreate, BooleanSetting, EpicRankRequest, EpicUpdate, FieldEdit, IssueAssignRequest, IssueRankRequest,
    SharePermissionInput, Sprint, SprintCreate, SprintSwap, UnmapSprints,
};

use super::fixtures::{
    board_of, create_epic, create_task, create_test_filter, property_body, property_value, scrum_project, touch,
};
use crate::harness::{ResourceTracker, server, test_name};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn creates_a_board_over_a_filter_of_its_own() {
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "board owner").await;
    let filter = create_test_filter(&mut tracker, "board filter", &format!("project = {}", project.key)).await;
    let filter_id = filter.id.clone().expect("a created filter carries an id");

    // A board needs a filter shared with someone; on a private instance that is everyone signed in.
    server()
        .filters()
        .add_share_permission(&filter_id)
        .share_permission_input(SharePermissionInput {
            r#type: Some("authenticated".to_owned()),
            ..SharePermissionInput::default()
        })
        .send()
        .await
        .expect("the filter can be shared with everyone signed in");

    let filter_id: i64 = filter_id.parse().expect("a filter id is a number");

    let name = test_name("a board");
    let board = server()
        .board()
        .create_board(BoardCreate {
            name: Some(name.clone()),
            r#type: Some("scrum".to_owned()),
            filter_id: Some(filter_id),
        })
        .send()
        .await
        .expect("a board can be created over a filter of the caller's own");

    let id = board.id.expect("a created board carries an id");

    tracker.defer(move || async move { server().board().delete_board(id).send().await });

    assert!(id > 0, "a board id is a number Jira assigned");
    assert_eq!(board.name.as_deref(), Some(name.as_str()), "and the board carries the name it was given");

    tracker.cleanup().await;
}

/// The board the Scrum template makes, everything readable about it, and the property it can hold.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_board_the_template_made() {
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "board reader").await;
    let board_id = board_of(&project.key).await;

    let board = server().board().get_board(board_id).send().await.expect("the board reads back by id");

    assert_eq!(board.id, Some(board_id), "the board read back is the one the template made");
    assert!(board.name.is_some_and(|name| !name.is_empty()), "and it is named");

    let configuration =
        server().board().get_board_configuration(board_id).send().await.expect("the configuration reads");

    assert_eq!(configuration.id, Some(board_id), "the configuration belongs to that board");
    assert!(configuration.filter.is_some(), "and it names the filter the board is built on");

    // Asked for by name rather than scanned out of the first page: an instance accumulates boards, the listing pages
    // at fifty, and a suite that reads page one is testing how recently the instance was cleaned.
    let boards =
        server().board().get_all_boards().name(&project.key).send().await.expect("the boards of an instance list");

    assert!(boards.values.iter().any(|board| board.id == Some(board_id)), "the board is in the instance listing");

    // Read unmodelled: the Data Center document declares this write as answering with `EntityPropertiesKeys`, and
    // the instance answers `null`. The gap is the document's — `jira.js` carries the same wrong return type and only
    // survives it because its default is to warn and hand the body back.
    server()
        .board()
        .set_board_property("suite", board_id, property_body())
        .send_raw()
        .await
        .expect("a board takes a property of the caller's own");

    let property =
        server().board().get_board_property("suite", board_id).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    server().board().delete_board_property("suite", board_id).send().await.expect("the property can be removed");

    // Refined velocity is a Data Center setting a board only carries where the Software application allows it.
    touch(server().board().set_refined_velocity(board_id, BooleanSetting { value: Some(true) }).send().await);

    let velocity = touch(server().board().get_refined_velocity(board_id).send().await);

    assert!(velocity.is_none_or(|velocity| velocity.value.is_some()), "the setting reads back as a boolean");

    tracker.cleanup().await;
}

/// A sprint, an issue moved into it and back out, and everything a sprint can be told to do while it exists.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn moves_an_issue_through_a_sprint_and_back_to_the_backlog() {
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "sprint owner").await;
    let board_id = board_of(&project.key).await;
    let issue = create_task(&mut tracker, &project.key, "an agile issue").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    let sprint = server()
        .sprint()
        .create_sprint(SprintCreate {
            name: Some(test_name("spr")),
            origin_board_id: Some(board_id),
            ..SprintCreate::default()
        })
        .send()
        .await
        .expect("a sprint can be created on the board");

    let sprint_id = sprint.id.expect("a created sprint carries an id");

    tracker.defer(move || async move { server().sprint().delete_sprint(sprint_id).send().await });

    server()
        .sprint()
        .move_issues_to_sprint(sprint_id, IssueAssignRequest { issues: Some(vec![key.clone()]) })
        .send()
        .await
        .expect("an issue can be moved into a sprint");

    let in_sprint = server().sprint().get_issues_for_sprint(sprint_id).send().await.expect("the sprint's issues read");

    assert!(
        in_sprint.issues.iter().flatten().any(|issue| issue.key.as_deref() == Some(key.as_str())),
        "the issue just moved is in the sprint",
    );

    server()
        .backlog()
        .move_issues_to_backlog(IssueAssignRequest { issues: Some(vec![key.clone()]) })
        .send()
        .await
        .expect("and can be sent back to the backlog");

    let after =
        server().sprint().get_issues_for_sprint(sprint_id).send().await.expect("the sprint's issues read again");

    assert!(
        !after.issues.iter().flatten().any(|issue| issue.key.as_deref() == Some(key.as_str())),
        "the move out of the sprint is observable on the next read",
    );

    server()
        .sprint()
        .set_sprint_property("suite", sprint_id, property_body())
        .send()
        .await
        .expect("a sprint takes a property of the caller's own");

    let property =
        server().sprint().get_sprint_property("suite", sprint_id).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    let keys = server().sprint().get_sprint_property_keys(sprint_id).send().await.expect("the property keys read");

    assert!(
        keys.keys.iter().flatten().any(|entry| entry.key.as_deref() == Some("suite")),
        "the key just written is in the listing",
    );

    server().sprint().delete_sprint_property("suite", sprint_id).send().await.expect("the property can be removed");

    let renamed = test_name("spr2");

    server()
        .sprint()
        .partially_update_sprint(sprint_id, Sprint { name: Some(renamed.clone()), ..Sprint::default() })
        .send()
        .await
        .expect("a sprint can be renamed in part");

    let read = server().sprint().get_sprint(sprint_id).send().await.expect("the renamed sprint reads back");

    assert_eq!(read.name.as_deref(), Some(renamed.as_str()), "the rename is observable on the next read");

    // A full update replaces the sprint, so every field it validates has to be present — `state` among them.
    let replaced = server()
        .sprint()
        .update_sprint(
            sprint_id,
            Sprint {
                name: Some(renamed),
                state: Some("future".to_owned()),
                goal: Some("proving the write path".to_owned()),
                ..Sprint::default()
            },
        )
        .send()
        .await
        .expect("a sprint can be replaced wholesale");

    assert_eq!(
        replaced.goal.as_deref(),
        Some("proving the write path"),
        "the replacement carries the goal it was given",
    );

    // Swapping and unmapping are for sprints a board no longer wants, which a single fresh sprint is not.
    touch(
        server()
            .sprint()
            .swap_sprint(sprint_id, SprintSwap { sprint_to_swap_with: Some(sprint_id), ..SprintSwap::default() })
            .send()
            .await,
    );
    touch(server().sprint().unmap_sprints(UnmapSprints { sprint_ids: Some(vec![sprint_id]) }).send().await);
    touch(server().sprint().unmap_all_sprints().send().await);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn moves_an_issue_into_an_epic_and_renames_it() {
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "epic owner").await;
    let epic = create_epic(&mut tracker, &project.key, "an epic").await;
    let issue = create_task(&mut tracker, &project.key, "an issue for the epic").await;
    let epic_key = epic.key.clone().expect("a created epic carries a key");
    let key = issue.key.clone().expect("a created issue carries a key");

    server()
        .epic()
        .move_issues_to_epic(&epic_key, IssueAssignRequest { issues: Some(vec![key.clone()]) })
        .send()
        .await
        .expect("an issue can be moved into an epic");

    let in_epic = server().epic().get_issues_for_epic(&epic_key).send().await.expect("the epic's issues read");

    assert!(
        in_epic.issues.iter().flatten().any(|issue| issue.key.as_deref() == Some(key.as_str())),
        "the issue just moved is in the epic",
    );

    server()
        .epic()
        .remove_issues_from_epic(IssueAssignRequest { issues: Some(vec![key]) })
        .send()
        .await
        .expect("and can be taken out again");

    let without = server().epic().get_issues_without_epic().send().await.expect("the epic-less issues read");

    assert!(
        without.issues.as_ref().is_some_and(|issues| issues.iter().all(|issue| issue.key.is_some())),
        "everything with no epic is still an issue with a key",
    );

    let renamed = test_name("renamed epic");
    let updated = server()
        .epic()
        .partially_update_epic(&epic_key, EpicUpdate { summary: Some(renamed.clone()), ..EpicUpdate::default() })
        .send()
        .await
        .expect("an epic can be renamed");

    assert_eq!(updated.summary.as_deref(), Some(renamed.as_str()), "the epic carries the summary it was given");

    // Ranking an epic against itself is the one arrangement Jira refuses, which is what makes this a `touch`.
    touch(
        server()
            .epic()
            .rank_epics(
                &epic_key,
                EpicRankRequest { rank_after_epic: Some(epic_key.clone()), ..EpicRankRequest::default() },
            )
            .send()
            .await,
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn ranks_and_estimates_an_issue() {
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "ranking owner").await;
    let board_id = board_of(&project.key).await;
    let issue = create_task(&mut tracker, &project.key, "an issue to rank").await;
    let other = create_task(&mut tracker, &project.key, "the issue it ranks against").await;
    let key = issue.key.clone().expect("a created issue carries a key");
    let other_key = other.key.clone().expect("a created issue carries a key");

    // Rank and estimate are stored in custom fields the Software application owns, and a board configured without
    // them refuses both — which is a refusal about configuration, not about the request.
    touch(
        server()
            .issues()
            .rank_issues(IssueRankRequest {
                issues: Some(vec![key.clone()]),
                rank_before_issue: Some(other_key),
                ..IssueRankRequest::default()
            })
            .send()
            .await,
    );
    touch(
        server()
            .issues()
            .estimate_issue_for_board(&key, FieldEdit { value: Some("5".to_owned()) })
            .board_id(board_id)
            .send()
            .await,
    );

    let estimate = touch(server().issues().get_issue_estimation_for_board(&key).board_id(board_id).send().await);

    assert!(
        estimate.is_none_or(|estimate| estimate.field_id.is_some() || estimate.value.is_some()),
        "an estimate names the field it is held in",
    );

    let agile_issue = server().issues().get_agile_issue(&key).send().await.expect("the agile view of an issue reads");

    assert_eq!(agile_issue.key.as_deref(), Some(key.as_str()), "the agile surface reads the same issue by key");

    tracker.cleanup().await;
}

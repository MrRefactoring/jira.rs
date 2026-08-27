//! The Agile `sprint` API, and the board-side sprint listing it is observed through.
//!
//! Sprints belong to a scrum board, so everything here is gated on one existing. Where it does, a sprint is created
//! and removed inside the run — it is board-scoped rather than site-wide, which makes it safe in a way most Agile
//! configuration is not.
//!
//! The behaviour that needs a live site is the state machine. A sprint is future, active or closed, transitions are
//! one-way, and the API expresses them as ordinary field updates — so nothing in the types stops a caller attempting
//! a transition that cannot happen.

use jira::agile::{GetAllSprintsRequestState, SprintState};

use crate::harness::{ResourceTracker, agile, await_readable, scrum_board, test_name};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_sprints_for_the_scrum_board() {
    let mut tracker = ResourceTracker::new();
    let board_id = scrum_board(&mut tracker).await;

    let sprints =
        agile().board().get_all_sprints(board_id).max_results(5).send().await.expect("the board lists its sprints");
    let values = sprints.values.expect("a sprint listing carries its values");

    assert!(sprints.is_last.is_some(), "a paged listing says whether it is the last page");
    assert!(values.len() <= 5, "five sprints were asked for, {} arrived", values.len());

    for sprint in &values {
        assert!(sprint.name.as_ref().is_some_and(|name| !name.is_empty()), "a sprint carries a name");
        assert!(
            sprint.state.is_documented(),
            "a sprint state outside the ones the specification lists: {:?}",
            sprint.state,
        );
    }

    tracker.cleanup().await;
}

/// The whole sprint lifecycle, as one sequence: every case below needs the sprint the one before it left behind, and
/// the closing case is the transition the state machine refuses.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_sprint_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let board_id = scrum_board(&mut tracker).await;

    // Jira truncates a sprint name past 30 characters, and a truncated name would not read back as the one written.
    let name: String = test_name("sprint").chars().take(30).collect();

    let created = agile()
        .sprint()
        .create_sprint(name.clone())
        .origin_board_id(board_id)
        .send()
        .await
        .expect("the scrum board accepts a new sprint");

    let sprint_id = created.id.expect("a created sprint carries an id");

    tracker.defer(move || async move { agile().sprint().delete_sprint(sprint_id).send().await });

    assert_eq!(created.state, SprintState::Future, "a sprint is born in the future state");
    assert_eq!(created.origin_board_id, Some(board_id), "a sprint knows the board it belongs to");

    let read = await_readable("the sprint reads back by id", || agile().sprint().get_sprint(sprint_id).send()).await;

    assert_eq!(read.id, Some(sprint_id));
    assert_eq!(read.name.as_deref(), Some(name.as_str()));

    let renamed = format!("{}-edited", name.chars().take(20).collect::<String>());

    agile()
        .sprint()
        .partially_update_sprint(sprint_id)
        .name(renamed.clone())
        .send()
        .await
        .expect("a sprint can be renamed through a partial update");

    let after_rename =
        await_readable("the renamed sprint reads back", || agile().sprint().get_sprint(sprint_id).send()).await;

    assert_eq!(after_rename.name.as_deref(), Some(renamed.as_str()));
    assert_eq!(after_rename.state, SprintState::Future, "renaming a sprint does not move it through its states");

    let future = agile()
        .board()
        .get_all_sprints(board_id)
        .state(GetAllSprintsRequestState::Future)
        .max_results(50)
        .send()
        .await
        .expect("the sprint listing takes a state filter");
    let future_ids: Vec<Option<i64>> = future.values.unwrap_or_default().into_iter().map(|sprint| sprint.id).collect();

    assert!(future_ids.contains(&Some(sprint_id)), "the state filter finds the sprint in the state it is in");

    let closed = agile()
        .board()
        .get_all_sprints(board_id)
        .state(GetAllSprintsRequestState::Closed)
        .max_results(50)
        .send()
        .await
        .expect("the sprint listing takes a state filter");
    let closed_ids: Vec<Option<i64>> = closed.values.unwrap_or_default().into_iter().map(|sprint| sprint.id).collect();

    assert!(!closed_ids.contains(&Some(sprint_id)), "and does not find it in a state it is not in");

    let issues =
        agile().sprint().get_issues_for_sprint(sprint_id).max_results(5).send().await.expect("a sprint lists issues");

    assert!(issues.issues.is_empty(), "a fresh sprint holds nothing");

    let error = agile()
        .sprint()
        .partially_update_sprint(sprint_id)
        .state("closed")
        .send()
        .await
        .expect_err("a sprint that was never started cannot be closed");

    assert_eq!(error.status(), Some(400), "the transition is refused by the state machine, not by permissions");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_sprint_as_not_found() {
    let error =
        agile().sprint().get_sprint(99_999_999).send().await.expect_err("a sprint that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

//! The Agile `board` API: the listing and its filters, a board's configuration, its projects and its issues.
//!
//! The Agile API is a different surface with its own base path, so this file exists first of all to prove that one
//! client reaches both — the same transport drives the platform calls and these.
//!
//! A board is not a container of its own: it is a saved filter over issues that live in projects. Boards are
//! therefore only read here, never created or deleted — deleting one strips a team of its working view.

use jira::agile::{BoardType, GetAllBoardsRequestType};

use crate::harness::{TEST_PROJECT_KEY, agile, cloud};

/// A board over the test project, where the site has one.
async fn any_board() -> Option<i64> {
    let boards = agile()
        .board()
        .get_all_boards()
        .project_key_or_id(TEST_PROJECT_KEY)
        .max_results(1)
        .send()
        .await
        .expect("the board listing is accepted");

    boards.values.first().and_then(|board| board.id)
}

/// The shape of every Jira issue key: a project key, a hyphen, a number.
fn is_issue_key(key: &str) -> bool {
    key.split_once('-').is_some_and(|(project, number)| {
        project.starts_with(|character: char| character.is_ascii_uppercase())
            && project.chars().all(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
            && !number.is_empty()
            && number.chars().all(|character| character.is_ascii_digit())
    })
}

/// One client, two base paths: the platform surface answers on `/rest/api/3` and the Agile one on `/rest/agile/1.0`,
/// and both are reached from the transport the harness builds once.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reaches_the_platform_and_the_agile_surface_from_one_client() {
    let (me, boards) = tokio::join!(
        cloud().myself().get_current_user().send(),
        agile().board().get_all_boards().max_results(1).send()
    );

    let me = me.expect("the platform surface answers");
    let boards = boards.expect("the Agile surface answers");

    assert!(me.account_id.is_some_and(|id| !id.is_empty()), "the current user carries an account id");
    assert_eq!(boards.max_results, 1, "the Agile surface honoured the page size the same request asked for");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_board_listing() {
    let boards = agile().board().get_all_boards().max_results(1).send().await.expect("the board listing is accepted");

    assert_eq!(boards.max_results, 1);
    assert!(boards.values.len() <= 1, "one board was asked for, {} arrived", boards.values.len());
    assert!(boards.total >= boards.values.len() as i64, "a page never reports fewer boards than it carries");

    for board in &boards.values {
        assert!(board.id.is_some(), "a board carries an id");
        assert!(board.name.as_ref().is_some_and(|name| !name.is_empty()), "a board carries a name");
        assert!(
            board.r#type.as_ref().is_some_and(BoardType::is_documented),
            "a board type outside the ones the specification lists: {:?}",
            board.r#type,
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_boards_by_type_and_by_project() {
    let by_type = agile()
        .board()
        .get_all_boards()
        .r#type(GetAllBoardsRequestType::Scrum)
        .max_results(50)
        .send()
        .await
        .expect("the type filter is accepted");

    assert!(
        by_type.values.iter().all(|board| board.r#type == Some(BoardType::Scrum)),
        "the type filter is applied: {:?}",
        by_type.values.iter().map(|board| board.r#type.clone()).collect::<Vec<_>>(),
    );

    let all = agile().board().get_all_boards().max_results(50).send().await.expect("the unfiltered listing answers");
    let by_project = agile()
        .board()
        .get_all_boards()
        .project_key_or_id(TEST_PROJECT_KEY)
        .max_results(50)
        .send()
        .await
        .expect("the project filter is accepted");

    assert!(by_project.total <= all.total, "filtering by project cannot widen the listing");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_a_board_and_the_projects_behind_it() {
    let Some(board_id) = any_board().await else {
        return;
    };

    let board = agile().board().get_board(board_id).send().await.expect("the board reads back by id");

    assert_eq!(board.id, Some(board_id));
    assert!(board.name.as_ref().is_some_and(|name| !name.is_empty()), "a board carries a name");

    let projects = agile().board().get_projects(board_id).send().await.expect("the board lists what it is built over");
    let values = projects.values.expect("a project listing carries its values");

    assert!(!values.is_empty(), "a board is a filter over at least one project");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn exposes_the_filter_and_columns_a_board_is_built_from() {
    let Some(board_id) = any_board().await else {
        return;
    };

    let configuration =
        agile().board().get_configuration(board_id).send().await.expect("the board reports its configuration");

    assert_eq!(configuration.id, Some(board_id));
    assert!(
        configuration.filter.and_then(|filter| filter.id).is_some_and(|id| !id.is_empty()),
        "a board is a saved filter, and says which one",
    );

    let columns = configuration.column_config.and_then(|config| config.columns).expect("a board reports its columns");

    assert!(!columns.is_empty(), "a board has at least one column to put issues in");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_issues_on_the_board() {
    let Some(board_id) = any_board().await else {
        return;
    };

    let issues =
        agile().board().get_issues_for_board(board_id).max_results(5).send().await.expect("the board lists its issues");

    assert!(issues.issues.len() <= 5, "five issues were asked for, {} arrived", issues.issues.len());
    assert!(
        issues.issues.iter().all(|issue| is_issue_key(&issue.key)),
        "every row on a board is an issue: {:?}",
        issues.issues.iter().map(|issue| issue.key.as_str()).collect::<Vec<_>>(),
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_board_as_not_found() {
    let error =
        agile().board().get_board(99_999_999).send().await.expect_err("a board that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

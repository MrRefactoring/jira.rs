//! The older status pair, `get_statuses` and `get_status`.
//!
//! Read-only by nature — this endpoint pair has no write half. What is worth asserting is the join: every status
//! carries a `statusCategory`, and that nested object is what a caller actually renders. A status whose category
//! failed to deserialize would still typecheck, because the model marks it optional.

use std::collections::HashSet;

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_every_status_on_the_site_each_joined_to_a_status_category() {
    let statuses = cloud().workflow_statuses().get_statuses().send().await.expect("the site lists its statuses");

    assert!(!statuses.is_empty(), "a site always carries the statuses its default workflow needs");

    for status in &statuses {
        let id = status.id.as_deref().expect("a status carries an id");

        assert!(!id.is_empty() && id.chars().all(|c| c.is_ascii_digit()), "an id is digits: {id}");
        assert!(status.name.as_ref().is_some_and(|name| !name.is_empty()), "a status carries a name");

        let category = status.status_category.as_ref().expect("a status is joined to a status category");

        assert!(category.key.as_ref().is_some_and(|key| !key.is_empty()), "the joined category carries a key");
    }
}

/// A board groups its columns by status category, so the terminal one has to be present or nothing renders as done.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn covers_the_categories_a_board_needs_to_render() {
    let statuses = cloud().workflow_statuses().get_statuses().send().await.expect("the site lists its statuses");

    let categories: HashSet<&str> = statuses
        .iter()
        .filter_map(|status| status.status_category.as_ref())
        .filter_map(|category| category.key.as_deref())
        .collect();

    assert!(categories.contains("done"), "a site carries at least one status in the done category, got {categories:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_status_by_id_and_by_name_alike() {
    let statuses = cloud().workflow_statuses().get_statuses().send().await.expect("the site lists its statuses");
    let sample = statuses.first().expect("a site carries at least one status");
    let id = sample.id.clone().expect("a status carries an id");
    let name = sample.name.clone().expect("a status carries a name");

    let by_id = cloud().workflow_statuses().get_status(id.as_str()).send().await.expect("a status resolves by id");
    let by_name =
        cloud().workflow_statuses().get_status(name.as_str()).send().await.expect("a status resolves by name");

    assert_eq!(by_id.id.as_deref(), Some(id.as_str()), "the id lookup answers with the status asked for");
    assert_eq!(by_name.name.as_deref(), Some(name.as_str()), "the name lookup answers with the status asked for");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn distinguishes_global_statuses_from_project_scoped_ones() {
    let statuses = cloud().workflow_statuses().get_statuses().send().await.expect("the site lists its statuses");

    for status in &statuses {
        let Some(kind) = status.scope.as_ref().and_then(|scope| scope.r#type.as_ref()) else { continue };

        assert!(
            matches!(kind.as_str(), "GLOBAL" | "PROJECT"),
            "a scope names one of the two kinds, got {}",
            kind.as_str(),
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_status_as_not_found() {
    let error = cloud()
        .workflow_statuses()
        .get_status("99999999")
        .send()
        .await
        .expect_err("a status that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

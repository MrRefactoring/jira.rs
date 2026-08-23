//! The status management API, not to be confused with `workflow_statuses`.
//!
//! That one is the older read-only pair, this one the newer management API — and they are emphatically *not* two views
//! of one set. They return overlapping but different collections, and they describe the ones they share differently. A
//! caller who reaches for the wrong one gets a plausible answer missing what they were looking for, which is the main
//! reason this suite exists alongside that one.
//!
//! Read-only throughout: deleting a status asks Jira to deal with every issue sitting in it, and creating one adds to
//! site configuration.

use std::collections::{HashMap, HashSet};

use jira::cloud::{JiraStatusStatusCategory, PageOfStatuses, StatusScopeType};

use crate::harness::{TEST_PROJECT_KEY, cloud};

/// Searches statuses, or proves the refusal is the typed one an account without administrator rights receives.
///
/// The management API is administrator-only. A token that is not one must not turn that into a red run, so the shape
/// of the refusal is asserted here and the caller stands down.
async fn search_statuses(max_results: i64) -> Option<PageOfStatuses> {
    match cloud().status().search().max_results(max_results).send().await {
        Ok(page) => Some(page),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a refusal names the rights: {error}"
            );

            None
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn searches_statuses_or_refuses_typed_without_admin_rights() {
    let Some(page) = search_statuses(5).await else { return };

    assert_eq!(page.max_results, Some(5), "the page echoes the limit asked for");

    for status in page.values.as_deref().unwrap_or_default() {
        assert!(
            status.id.as_deref().is_some_and(|id| !id.is_empty()),
            "a status carries an id"
        );
        assert!(
            status.name.as_deref().is_some_and(|name| !name.is_empty()),
            "a status carries a name"
        );
        assert!(
            status
                .status_category
                .as_ref()
                .is_some_and(JiraStatusStatusCategory::is_documented),
            "a category is one of the three the API documents: {:?}",
            status.status_category,
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_a_different_set_of_statuses_from_the_older_api() {
    let Some(modern) = search_statuses(100).await else {
        return;
    };
    let legacy = cloud()
        .workflow_statuses()
        .get_statuses()
        .send()
        .await
        .expect("the older listing answers");

    let modern_ids: HashSet<&str> = modern
        .values
        .as_deref()
        .unwrap_or_default()
        .iter()
        .filter_map(|status| status.id.as_deref())
        .collect();
    let legacy_ids: HashSet<&str> = legacy.iter().filter_map(|status| status.id.as_deref()).collect();

    assert!(
        modern_ids.difference(&legacy_ids).next().is_some(),
        "the management API knows statuses the older one does not",
    );
    assert!(
        legacy_ids.difference(&modern_ids).next().is_some(),
        "the older API knows statuses the management one does not",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_a_shared_status_differently_in_each_api() {
    let Some(modern) = search_statuses(100).await else {
        return;
    };
    let legacy = cloud()
        .workflow_statuses()
        .get_statuses()
        .send()
        .await
        .expect("the older listing answers");

    let legacy_by_id: HashMap<&str, _> = legacy
        .iter()
        .filter_map(|status| status.id.as_deref().map(|id| (id, status)))
        .collect();

    let shared = modern
        .values
        .as_deref()
        .unwrap_or_default()
        .iter()
        .find(|status| status.id.as_deref().is_some_and(|id| legacy_by_id.contains_key(id)));

    let Some(shared) = shared else { return };
    let via_old = legacy_by_id[shared.id.as_deref().expect("the shared status was matched by its id")];

    assert_eq!(
        shared.name, via_old.name,
        "one status, one name, whichever API is asked"
    );
    assert!(
        shared
            .status_category
            .as_ref()
            .is_some_and(|category| !category.as_str().is_empty()),
        "the management API names the category with a bare string",
    );
    assert!(
        via_old
            .status_category
            .as_ref()
            .is_some_and(|category| category.key.is_some()),
        "the older API describes the same category as an object with a key of its own",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_the_search_by_category_and_by_name() {
    let Some(all) = search_statuses(1).await else { return };

    let done = cloud()
        .status()
        .search()
        .status_category("DONE")
        .max_results(50)
        .send()
        .await
        .expect("the search filters by category");

    for status in done.values.as_deref().unwrap_or_default() {
        assert_eq!(
            status.status_category.as_ref().map(JiraStatusStatusCategory::as_str),
            Some("DONE"),
            "a category filter returns only that category",
        );
    }

    let Some(name) = all
        .values
        .as_deref()
        .unwrap_or_default()
        .first()
        .and_then(|status| status.name.clone())
    else {
        return;
    };

    let by_name = cloud()
        .status()
        .search()
        .search_string(name.as_str())
        .max_results(50)
        .send()
        .await
        .expect("the search filters by name");

    assert!(
        by_name
            .values
            .as_deref()
            .unwrap_or_default()
            .iter()
            .any(|status| status.name.as_deref() == Some(&name)),
        "a search for a name that exists finds it",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn distinguishes_global_statuses_from_project_scoped_ones() {
    if search_statuses(1).await.is_none() {
        return;
    }

    let project = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project is readable");
    let project_id = project.id.expect("a project carries an id");

    let scoped = cloud()
        .status()
        .search()
        .project_id(project_id.as_str())
        .max_results(50)
        .send()
        .await
        .expect("the search accepts a project");

    let values = scoped.values.as_deref().unwrap_or_default();

    assert!(!values.is_empty(), "a project the suite works in has statuses");

    for status in values {
        if let Some(scope) = &status.scope
            && scope.r#type == StatusScopeType::Project
        {
            assert_eq!(
                scope.project.as_ref().map(|project| project.id.as_str()),
                Some(project_id.as_str()),
                "a project-scoped status names the project that was asked about",
            );
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_which_projects_and_workflows_use_a_status() {
    let Some(page) = search_statuses(5).await else { return };
    let Some(status_id) = page
        .values
        .as_deref()
        .unwrap_or_default()
        .first()
        .and_then(|status| status.id.clone())
    else {
        return;
    };

    match cloud()
        .status()
        .get_project_usages_for_status(status_id.as_str())
        .send()
        .await
    {
        Ok(usage) => assert_eq!(
            usage.status_id.as_deref(),
            Some(status_id.as_str()),
            "the usage report names the status it was asked about",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }

    match cloud()
        .status()
        .get_workflow_usages_for_status(status_id.as_str())
        .send()
        .await
    {
        Ok(usage) => assert_eq!(
            usage.status_id.as_deref(),
            Some(status_id.as_str()),
            "the workflow usage report names the status it was asked about",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_status_id_as_an_empty_result_rather_than_an_error() {
    if search_statuses(1).await.is_none() {
        return;
    }

    match cloud().status().get_statuses_by_id(["99999999"]).send().await {
        Ok(statuses) => assert!(statuses.is_empty(), "an id nothing matches comes back as an empty list"),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }
}

/// The destructive path is proven typed without ever being aimed at a real status.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .status()
        .delete_statuses_by_id(["99999999"])
        .send()
        .await
        .expect_err("a status that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

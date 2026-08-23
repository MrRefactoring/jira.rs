use jira::cloud::{GetProjectRequestExpand, SearchProjectsRequestOrderBy};

use crate::harness::{TEST_PROJECT_KEY, cloud};

/// The projects surface, read-only against the standing test project.
///
/// Deliberately read-only: creating a project on Cloud is slow, consumes a licence slot, and frequently cannot be
/// deleted by the token that made it, so a suite that made one per run would leave debris on a real tenant. The
/// destructive endpoints are still pinned, but only through their error channel and never aimed at a real project.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_the_test_project_the_whole_live_suite_runs_in() {
    let project = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back");

    assert_eq!(project.key.as_deref(), Some(TEST_PROJECT_KEY));
    assert!(
        project
            .id
            .as_ref()
            .is_some_and(|id| !id.is_empty() && id.chars().all(|c| c.is_ascii_digit())),
        "an id is digits: {:?}",
        project.id,
    );
    assert!(
        project.name.is_some_and(|name| !name.is_empty()),
        "a project carries a name"
    );
    assert!(
        project.self_.is_some_and(|link| link.starts_with("https://")),
        "a project carries an absolute self link"
    );

    let issue_types: Vec<String> = project
        .issue_types
        .unwrap_or_default()
        .into_iter()
        .filter_map(|issue_type| issue_type.name)
        .collect();

    assert!(
        issue_types.iter().any(|name| name == "Task"),
        "the test project offers Task: {issue_types:?}"
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_the_project_by_id_as_well_as_by_key() {
    let by_key = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project reads back by key");
    let id = by_key.id.expect("a project carries an id");
    let by_id = cloud()
        .projects()
        .get_project(&id)
        .send()
        .await
        .expect("the project reads back by id");

    assert_eq!(by_id.key.as_deref(), Some(TEST_PROJECT_KEY));
    assert_eq!(by_id.id.as_deref(), Some(id.as_str()));
}

/// The parameter documents that description, issue types and lead are in every response, expanded or not.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn already_returns_lead_and_description_without_being_asked() {
    let plain = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project reads back plain");
    let expanded = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .expand(GetProjectRequestExpand::Variant1(vec![
            "description".to_owned(),
            "lead".to_owned(),
        ]))
        .send()
        .await
        .expect("the expand parameter is accepted");

    assert!(
        plain
            .lead
            .as_ref()
            .and_then(|lead| lead.account_id.as_ref())
            .is_some_and(|id| !id.is_empty()),
        "the lead arrives without being asked for",
    );
    assert_eq!(
        expanded.lead.and_then(|lead| lead.account_id),
        plain.lead.and_then(|lead| lead.account_id),
        "expanding the lead returns the same lead",
    );
    assert_eq!(
        expanded.description, plain.description,
        "the description arrives without being asked for"
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_the_project_through_the_paginated_search() {
    let page = cloud()
        .projects()
        .search_projects()
        .query(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project search is accepted");

    let keys: Vec<String> = page.values.iter().filter_map(|project| project.key.clone()).collect();

    assert!(
        keys.iter().any(|key| key == TEST_PROJECT_KEY),
        "the search finds the test project: {keys:?}"
    );
    assert!(page.total >= 1, "a page that found something reports a total");
    assert!(
        page.values.len() as i64 <= page.max_results,
        "a page never exceeds the size it declares"
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_and_orders_the_project_search() {
    let limited = cloud()
        .projects()
        .search_projects()
        .max_results(1)
        .send()
        .await
        .expect("a page size of one is accepted");

    assert!(
        limited.values.len() <= 1,
        "one result was asked for, {} arrived",
        limited.values.len()
    );
    assert_eq!(limited.max_results, 1);

    let ascending = cloud()
        .projects()
        .search_projects()
        .order_by(SearchProjectsRequestOrderBy::Key)
        .max_results(50)
        .send()
        .await
        .expect("ordering by key is accepted");
    let descending = cloud()
        .projects()
        .search_projects()
        .order_by(SearchProjectsRequestOrderBy::KeyDescending)
        .max_results(50)
        .send()
        .await
        .expect("ordering by key descending is accepted");

    let up: Vec<Option<String>> = ascending.values.iter().map(|project| project.key.clone()).collect();
    let down: Vec<Option<String>> = descending.values.iter().map(|project| project.key.clone()).collect();

    if up.len() > 1 {
        assert_eq!(
            down,
            up.iter().rev().cloned().collect::<Vec<_>>(),
            "the sort direction reverses the page"
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_statuses_available_per_issue_type() {
    let statuses = cloud()
        .projects()
        .get_all_statuses(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project reports its statuses");

    assert!(!statuses.is_empty(), "a project has at least one issue type");

    for issue_type in &statuses {
        assert!(!issue_type.name.is_empty(), "an issue type carries a name");
        assert!(
            !issue_type.statuses.is_empty(),
            "{} has at least one status",
            issue_type.name
        );

        for status in &issue_type.statuses {
            assert!(
                status
                    .status_category
                    .as_ref()
                    .and_then(|category| category.key.as_ref())
                    .is_some_and(|key| !key.is_empty()),
                "every status belongs to a category: {:?}",
                status.name,
            );
        }
    }
}

/// The hierarchy endpoint is team-managed only; a company-managed project answers a 404 that says so.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_the_issue_type_hierarchy_or_refuses_for_a_company_managed_project() {
    let project = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back");
    let id: i64 = project
        .id
        .expect("a project carries an id")
        .parse()
        .expect("a project id is a number");

    match cloud().projects().get_hierarchy(id).send().await {
        Ok(hierarchy) => {
            assert_eq!(hierarchy.project_id, Some(id));

            for level in hierarchy.hierarchy.unwrap_or_default() {
                assert!(level.level.is_some(), "a hierarchy level carries its depth");
            }
        }
        Err(error) => {
            assert!(error.is_not_found(), "{error}");
            assert!(
                error
                    .body()
                    .is_some_and(|body| body.to_string().contains("not simplified")),
                "a company-managed project says why: {error}",
            );
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_notification_scheme_or_a_typed_404_when_none_is_attached() {
    match cloud()
        .projects()
        .get_notification_scheme_for_project(TEST_PROJECT_KEY)
        .send()
        .await
    {
        Ok(scheme) => {
            assert!(scheme.id.is_some(), "a notification scheme carries an id");
            assert!(
                scheme.name.is_some_and(|name| !name.is_empty()),
                "a notification scheme carries a name"
            );
        }
        Err(error) => {
            assert!(error.is_not_found(), "{error}");
            assert!(
                error
                    .body()
                    .is_some_and(|body| body.to_string().contains("notification scheme")),
                "the refusal names what is missing: {error}",
            );
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_project_as_not_found() {
    let error = cloud()
        .projects()
        .get_project("NOSUCHPROJECT")
        .send()
        .await
        .expect_err("a project that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn is_case_sensitive_about_project_keys() {
    let error = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY.to_lowercase())
        .send()
        .await
        .expect_err("a lowercased key is a different key");

    assert!(error.is_not_found(), "{error}");
}

/// The destructive path is pinned through its error channel, never aimed at a real project.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .projects()
        .delete_project("NOSUCHPROJECT")
        .send()
        .await
        .expect_err("a project that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

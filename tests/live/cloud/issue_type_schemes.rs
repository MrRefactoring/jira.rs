use jira::cloud::GetAllIssueTypeSchemesRequestOrderBy;

use crate::harness::{TEST_ISSUE_TYPE, TEST_PROJECT_KEY, cloud};

/// Issue type schemes, read-only.
///
/// A scheme decides which issue types a project offers, and schemes are shared: adding the Epic type to the test
/// project would mean editing a scheme other projects depend on, which is why no suite here does it. This file is
/// the other half of that story — it shows the mapping that made the decision.
///
/// Every read needs *Administer Jira*, so each test first proves the token either has it or is refused typed.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_scheme_listing_for_an_admin_or_fails_typed() {
    if !may_read_issue_type_schemes().await {
        return;
    }

    let page = cloud()
        .issue_type_schemes()
        .get_all_issue_type_schemes()
        .max_results(5)
        .send()
        .await
        .expect("an admin lists the issue type schemes");

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");

    for scheme in &page.values {
        assert!(!scheme.id.is_empty(), "a scheme carries an id");
        assert!(!scheme.name.is_empty(), "a scheme carries a name");

        if let Some(default) = scheme.default_issue_type_id.as_deref() {
            assert!(
                !default.is_empty() && default.bytes().all(|byte| byte.is_ascii_digit()),
                "a default issue type is named by its numeric id: {default}",
            );
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_the_scheme_the_test_project_is_attached_to() {
    if !may_read_issue_type_schemes().await {
        return;
    }

    let project_id = test_project_id().await;

    let page = cloud()
        .issue_type_schemes()
        .get_issue_type_scheme_for_projects([project_id])
        .send()
        .await
        .expect("the project resolves to its issue type scheme");

    assert_eq!(page.values.len(), 1, "a project is attached to exactly one issue type scheme");

    let entry = &page.values[0];

    assert!(
        entry.project_ids.iter().any(|id| id.parse::<i64>() == Ok(project_id)),
        "the entry names the project it was asked about, got {:?}",
        entry.project_ids,
    );
    assert!(
        entry.issue_type_scheme.as_ref().is_some_and(|scheme| !scheme.id.is_empty()),
        "the entry names the scheme, which is the whole point of the lookup",
    );
}

/// The mapping is the authority on what a project offers, so it has to agree with the project itself. A scheme that
/// listed types the project does not show — or the reverse — is the exact failure that makes a create dialog wrong.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn explains_which_issue_types_the_project_offers() {
    if !may_read_issue_type_schemes().await {
        return;
    }

    let project_id = test_project_id().await;

    let for_project = cloud()
        .issue_type_schemes()
        .get_issue_type_scheme_for_projects([project_id])
        .send()
        .await
        .expect("the project resolves to its issue type scheme");

    let scheme_id: i64 = for_project
        .values
        .first()
        .and_then(|entry| entry.issue_type_scheme.as_ref())
        .expect("the project names a scheme")
        .id
        .parse()
        .expect("a scheme id is a number");

    let mapping = cloud()
        .issue_type_schemes()
        .get_issue_type_schemes_mapping()
        .issue_type_scheme_id([scheme_id])
        .max_results(100)
        .send()
        .await
        .expect("the scheme lists the issue types it maps");

    let mut mapped: Vec<&str> = mapping.values.iter().map(|entry| entry.issue_type_id.as_str()).collect();

    mapped.sort_unstable();

    let project = cloud().projects().get_project(TEST_PROJECT_KEY).send().await.expect("the test project reads back");
    let issue_types = project.issue_types.as_ref().expect("a project lists the issue types it offers");
    let mut offered: Vec<&str> = issue_types.iter().filter_map(|issue_type| issue_type.id.as_deref()).collect();

    offered.sort_unstable();

    assert_eq!(mapped, offered, "the scheme mapping is exactly what the project offers, in both directions");
    assert!(
        issue_types.iter().any(|issue_type| issue_type.name.as_deref() == Some(TEST_ISSUE_TYPE)),
        "the project offers the type every Cloud suite creates issues with",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn orders_the_listing_by_name_in_both_directions() {
    if !may_read_issue_type_schemes().await {
        return;
    }

    let ascending = cloud()
        .issue_type_schemes()
        .get_all_issue_type_schemes()
        .order_by(GetAllIssueTypeSchemesRequestOrderBy::Name)
        .max_results(50)
        .send()
        .await
        .expect("the ascending order is accepted");

    let descending = cloud()
        .issue_type_schemes()
        .get_all_issue_type_schemes()
        .order_by(GetAllIssueTypeSchemesRequestOrderBy::NameDescending)
        .max_results(50)
        .send()
        .await
        .expect("the descending order is accepted");

    assert_eq!(descending.values.len(), ascending.values.len(), "reversing the sort does not change what is listed");

    if ascending.values.len() <= 1 {
        return;
    }

    let forwards: Vec<&str> = ascending.values.iter().map(|scheme| scheme.id.as_str()).collect();
    let mut backwards: Vec<&str> = descending.values.iter().map(|scheme| scheme.id.as_str()).collect();

    backwards.reverse();

    assert_eq!(forwards, backwards, "reversing the sort reverses the page");
}

/// The destructive path, proven through its error channel and never aimed at a scheme that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_type_schemes()
        .delete_issue_type_scheme(99_999_999)
        .send()
        .await
        .expect_err("an issue type scheme that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The numeric id of the project every Cloud suite works in, which is what the scheme lookups take.
async fn test_project_id() -> i64 {
    cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back")
        .id
        .expect("a project carries an id")
        .parse()
        .expect("a project id is a number")
}

/// Whether the token may read the issue type scheme configuration at all.
///
/// A token without *Administer Jira* must be refused in a way the caller can recognise, so the refusal is asserted
/// here rather than being silently swallowed by the tests that stand down on it.
async fn may_read_issue_type_schemes() -> bool {
    match cloud().issue_type_schemes().get_all_issue_type_schemes().max_results(1).send().await {
        Ok(_) => true,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            false
        }
    }
}

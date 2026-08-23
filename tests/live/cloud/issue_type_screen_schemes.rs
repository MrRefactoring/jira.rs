use crate::harness::{TEST_PROJECT_KEY, cloud};

/// Issue type screen schemes, read-only.
///
/// This is the top of the chain the screen scheme suite walks: a project has one issue type screen scheme, which
/// maps each issue type to a screen scheme, which maps each operation to a screen. Reassigning it changes every form
/// in the project at once, so nothing here writes.
///
/// It is covered apart from the screen schemes for one reason: this is the only layer that is project-associated, so
/// it is where "which forms does *this* project use" is actually answered.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_listing_for_an_admin_or_fails_typed() {
    if !may_read_issue_type_screen_schemes().await {
        return;
    }

    let page = cloud()
        .issue_type_screen_schemes()
        .get_issue_type_screen_schemes()
        .max_results(5)
        .send()
        .await
        .expect("an admin lists the issue type screen schemes");

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");

    for scheme in &page.values {
        assert!(!scheme.id.is_empty(), "a scheme carries an id");
        assert!(!scheme.name.is_empty(), "a scheme carries a name");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_the_scheme_the_test_project_is_associated_with() {
    if !may_read_issue_type_screen_schemes().await {
        return;
    }

    let project_id = test_project_id().await;

    let page = match cloud()
        .issue_type_screen_schemes()
        .get_issue_type_screen_scheme_project_associations([project_id])
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused association lookup is typed: {error}");

            return;
        }
    };

    assert_eq!(page.values.len(), 1, "a project is associated with exactly one issue type screen scheme");

    let association = &page.values[0];

    assert!(
        association.issue_type_screen_scheme.as_ref().is_some_and(|scheme| !scheme.id.is_empty()),
        "the association names the scheme, which is the whole point of the lookup",
    );
    assert!(
        association.project_ids.iter().any(|id| id.parse::<i64>() == Ok(project_id)),
        "the association names the project it was asked about, got {:?}",
        association.project_ids,
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn maps_issue_types_to_screen_schemes() {
    if !may_read_issue_type_screen_schemes().await {
        return;
    }

    let Some(scheme_id) = project_scheme_id().await else { return };

    let mappings = cloud()
        .issue_type_screen_schemes()
        .get_issue_type_screen_scheme_mappings()
        .issue_type_screen_scheme_id([scheme_id])
        .max_results(50)
        .send()
        .await
        .expect("the scheme lists the mappings it holds");

    for mapping in &mappings.values {
        assert!(!mapping.issue_type_id.is_empty(), "a mapping names an issue type");
        assert!(!mapping.screen_scheme_id.is_empty(), "a mapping names the screen scheme that issue type resolves to");
        assert_eq!(
            mapping.issue_type_screen_scheme_id.parse::<i64>(),
            Ok(scheme_id),
            "a mapping belongs to the scheme it was asked for",
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_projects_a_scheme_is_used_by() {
    if !may_read_issue_type_screen_schemes().await {
        return;
    }

    let project_id = test_project_id().await;
    let Some(scheme_id) = project_scheme_id().await else { return };

    let projects = match cloud()
        .issue_type_screen_schemes()
        .get_projects_for_issue_type_screen_scheme(scheme_id)
        .max_results(50)
        .send()
        .await
    {
        Ok(projects) => projects,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused project listing is typed: {error}");

            return;
        }
    };

    let listed: Vec<i64> =
        projects.values.iter().filter_map(|project| project.id.as_deref()?.parse::<i64>().ok()).collect();

    assert!(
        listed.contains(&project_id),
        "the association reads the same way round: the scheme lists the project that named it, got {listed:?}",
    );
}

/// The destructive path, proven through its error channel and never aimed at a scheme that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_type_screen_schemes()
        .delete_issue_type_screen_scheme("99999999")
        .send()
        .await
        .expect_err("an issue type screen scheme that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The numeric id of the project every Cloud suite works in, which is what the association lookups take.
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

/// The issue type screen scheme the test project is associated with, or nothing when the site refuses the lookup.
///
/// A refusal is asserted to be typed here, so a test that stands down on it has still proven something.
async fn project_scheme_id() -> Option<i64> {
    let project_id = test_project_id().await;

    let page = match cloud()
        .issue_type_screen_schemes()
        .get_issue_type_screen_scheme_project_associations([project_id])
        .send()
        .await
    {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused association lookup is typed: {error}");

            return None;
        }
    };

    page.values
        .first()
        .and_then(|association| association.issue_type_screen_scheme.as_ref())
        .map(|scheme| scheme.id.parse().expect("a scheme id is a number"))
}

/// Whether the token may read the issue type screen scheme configuration at all.
///
/// A token without *Administer Jira* must be refused in a way the caller can recognise, so the refusal is asserted
/// here rather than being silently swallowed by the tests that stand down on it.
async fn may_read_issue_type_screen_schemes() -> bool {
    match cloud().issue_type_screen_schemes().get_issue_type_screen_schemes().max_results(1).send().await {
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

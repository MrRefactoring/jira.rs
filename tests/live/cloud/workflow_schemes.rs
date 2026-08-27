//! Workflow schemes and the projects they are bound to.
//!
//! Read-only. A workflow scheme binds issue types to workflows for a project, and one scheme serves many projects —
//! reassigning it changes how issues move for all of them, and Jira asks for a migration when in-flight issues no
//! longer have a valid status. So the writes are pinned only through their error channel, aimed at an id nothing can
//! match.
//!
//! The legacy listing and the newer `read_workflow_schemes` are both exercised. The newer one is the one that used to
//! break: its response declares `description` as a plain string, and a model that made it an ADF document turned every
//! call into a deserialization failure. That is why the read half is asserted not to be a schema mismatch rather than
//! merely asserted to fail.

use jira::cloud::{Page, WorkflowScheme, WorkflowSchemeReadRequest};

use crate::harness::{TEST_PROJECT_KEY, await_readable, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_scheme_listing_for_an_admin_or_fails_typed() {
    let Some(page) = list_schemes(5).await else { return };

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");

    for scheme in &page.values {
        assert!(scheme.id.is_some_and(|id| id > 0), "a scheme carries an id");
        assert!(scheme.name.as_ref().is_some_and(|name| !name.is_empty()), "a scheme carries a name");
        assert!(
            scheme.default_workflow.as_ref().is_none_or(|workflow| !workflow.is_empty()),
            "a scheme that names a default workflow names it with a non-empty string",
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_the_scheme_the_test_project_is_associated_with() {
    if list_schemes(1).await.is_none() {
        return;
    }

    let project_id = test_project_id().await;

    let associations = match cloud()
        .workflow_scheme_project_associations()
        .get_workflow_scheme_project_associations([project_id])
        .send()
        .await
    {
        Ok(associations) => associations,
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused association read is typed: {error}");

            return;
        }
    };

    for association in &associations.values {
        assert!(
            association.project_ids.iter().any(|id| id == &project_id.to_string()),
            "an association lists the project it was asked about, got {:?}",
            association.project_ids,
        );
    }
}

/// The mapping is what a scheme *is*: an issue type either has its own workflow or falls through to the default one.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn maps_issue_types_to_workflows_within_a_scheme() {
    let Some(page) = list_schemes(1).await else { return };
    let Some(scheme) = page.values.first() else { return };
    let scheme_id = scheme.id.expect("a scheme carries an id");

    let detail = match cloud().workflow_schemes().get_workflow_scheme(scheme_id).send().await {
        Ok(detail) => detail,
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused scheme read is typed: {error}");

            return;
        }
    };

    assert_eq!(detail.id, Some(scheme_id), "the detail answers with the scheme it was asked about");
    assert!(
        detail.default_workflow.is_some() || detail.issue_type_mappings.is_some(),
        "a scheme names a default workflow, per-issue-type mappings, or both",
    );
}

/// The newer read endpoint, which is where the string-versus-document defect lived.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_schemes_through_the_newer_endpoint_without_a_schema_mismatch() {
    let project_id = test_project_id().await;

    let request = WorkflowSchemeReadRequest {
        project_ids: Some(vec![Some(project_id.to_string())]),
        ..WorkflowSchemeReadRequest::default()
    };

    match cloud().workflow_schemes().read_workflow_schemes(request).send().await {
        Ok(schemes) => {
            for scheme in &schemes {
                assert!(!scheme.id.is_empty(), "a scheme carries an id");
                assert!(!scheme.name.is_empty(), "a scheme carries a name");
                // That the description is a `String` at all is the point: the older endpoint models it as a
                // document. Emptiness is the site's business — a scheme nobody described has an empty one.
                assert!(scheme.description.is_some(), "the newer endpoint reports a description, as a plain string",);
            }
        }
        Err(error) => {
            assert!(!error.is_schema_mismatch(), "the response model matches what the endpoint sends: {error}");
            assert!(error.status().is_some_and(|status| status >= 400), "a refused read is typed: {error}");
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_scheme_as_a_typed_error() {
    let error = cloud()
        .workflow_schemes()
        .get_workflow_scheme(99_999_999)
        .send()
        .await
        .expect_err("a scheme that does not exist cannot be read");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}

/// The destructive path, proven typed and never aimed at a scheme that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .workflow_schemes()
        .delete_workflow_scheme(99_999_999)
        .send()
        .await
        .expect_err("a scheme that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The numeric id of the project every suite works in, which is what the association endpoints take.
async fn test_project_id() -> i64 {
    let project =
        await_readable("the test project is readable", || cloud().projects().get_project(TEST_PROJECT_KEY).send())
            .await;
    let id = project.id.expect("a project carries an id");

    id.parse().expect("a project id is a number")
}

/// Whether the token may read the workflow scheme configuration at all.
///
/// Every listing here needs *Administer Jira*. A token without it must be refused in a way the caller can recognise,
/// so the refusal is asserted here rather than being silently swallowed by the tests that stand down on it.
async fn list_schemes(max_results: i64) -> Option<Page<WorkflowScheme>> {
    match cloud().workflow_schemes().get_all_workflow_schemes().max_results(max_results).send().await {
        Ok(page) => Some(page),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            None
        }
    }
}

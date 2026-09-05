use crate::harness::service_desk;

/// The Service Management `assets` API, which is two endpoints that once did the same thing under two names.
///
/// Insight was renamed Assets and the older path was kept for compatibility. It no longer is: on Cloud today
/// `/assets/workspace` answers and `/insight/workspace` is gone, so what used to be worth asserting about the pair —
/// that they behave identically — is simply false, and these pin the deprecation instead.
///
/// Both are gated behind the same agent licence as the rest of the surface, so an instance without one refuses rather
/// than answers, and either outcome is accepted as long as the refusal is typed.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_the_assets_workspace_lookup_or_refuses_typed() {
    let page = match service_desk().assets().get_assets_workspaces().limit(5).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(404),
                "the workspace lookup is refused by licence or absent, not untyped: {error}",
            );

            return;
        }
    };

    assert!(page.values.len() <= 5, "a page holds no more than the limit asked for: {}", page.values.len());

    for workspace in &page.values {
        assert!(
            workspace.workspace_id.as_ref().is_some_and(|id| !id.is_empty()),
            "a workspace is identified by the id the Assets REST API is reached with",
        );
    }
}

/// The retired path answers `403` with an HTML error page rather than the JSON every other refusal on this surface
/// carries. That is what makes this worth keeping now that the paths have diverged: it is the only live assertion
/// that a non-JSON body still arrives as a typed error with its status intact, instead of failing inside the response
/// parser.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
#[allow(deprecated, reason = "the point of the case is that the retired path still answers with a typed error")]
async fn refuses_the_retired_insight_path_with_a_typed_error_whatever_the_body() {
    let error = match service_desk().assets().get_insight_workspaces().limit(1).send().await {
        Ok(page) => {
            assert!(page.values.len() <= 1, "a page holds no more than the limit asked for: {}", page.values.len());

            return;
        }
        Err(error) => error,
    };

    assert!(error.status().is_some(), "an HTML error body still carries its status through: {error}");
    assert!(!error.is_serialization(), "the non-JSON body is classified, not left to the response parser: {error}");
    assert!(
        error.is_forbidden() || error.status() == Some(404),
        "the retired path refuses by licence or is gone entirely: {error}",
    );
}

use crate::harness::service_desk;

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

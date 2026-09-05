//! The Assets Cloud API.
//!
//! Assets needs Jira Service Management Premium. On a site without it the workspace lookup answers with an empty page,
//! and there is nothing to point a client at — so this stands down, visibly, rather than failing five times over a
//! plan. It comes back the moment the site is on Premium, without an edit here.
//!
//! The client is deliberately not the shared one. Assets is the surface that answers neither on the site's own host
//! nor on a bare gateway, but under a path built from the workspace id — and that this suite has to build its own
//! transport to reach it is itself the thing worth showing a reader.

use jira::assets::{AssetsClient, ObjectAQLTotalCountParams};
use jira::{Auth, Client};

use crate::harness::{require_live_env, service_desk};

/// The workspace the site's Service Management gives Assets, when it has one.
///
/// Both ways of having none assert on the way past: a site with no Service Management at all refuses the lookup
/// rather than answering it empty, and a site on a lesser plan answers a page that says it is the only one.
async fn workspace_id() -> Option<String> {
    match service_desk().assets().get_assets_workspaces().limit(1).send().await {
        Ok(page) => {
            if let Some(workspace) = page.values.first().and_then(|workspace| workspace.workspace_id.clone()) {
                return Some(workspace);
            }

            assert_ne!(page.is_last_page, Some(false), "a page with no workspaces on it is the last page");

            None
        }
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "the site refused rather than failed: {error}");

            None
        }
    }
}

/// A transport addressed at one Assets workspace, which is where every operation below lives.
fn assets(workspace: &str) -> AssetsClient {
    let env = require_live_env();

    AssetsClient::new(
        Client::builder()
            .host(format!("https://api.atlassian.com/jsm/assets/workspace/{workspace}/v1"))
            .auth(Auth::api_token(env.email, env.api_token))
            .build()
            .expect("the live credentials describe a usable Assets client"),
    )
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_whether_the_site_has_an_assets_workspace() {
    // The gate every other test here runs behind, asserted once in its own right rather than only as a side effect.
    let Some(workspace) = workspace_id().await else { return };

    assert!(!workspace.is_empty(), "a workspace is named by an id");
    assert!(
        workspace.chars().all(|character| character.is_ascii_hexdigit() || character == '-'),
        "an Assets workspace id is a UUID: {workspace}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_object_schemas_of_the_workspace() {
    let Some(workspace) = workspace_id().await else { return };

    let schemas =
        assets(&workspace).object_schemas().find_schemas().max_results(5).send().await.expect("the workspace answers");

    assert_eq!(schemas.max_results, 5, "the page size asked for is the page size returned");
    assert!(schemas.values.len() <= 5, "a page holds no more than it says it does");
    assert!(schemas.total >= schemas.values.len() as i64, "the total covers the page that was just read");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_global_icons() {
    let Some(workspace) = workspace_id().await else { return };

    let icons = assets(&workspace).icons().find_global_icons().send().await.expect("the workspace lists its icons");

    assert!(!icons.is_empty(), "Assets ships global icons with every workspace");
    assert!(
        icons.iter().all(|icon| !icon.id.is_empty() && !icon.name.is_empty() && icon.url16.starts_with("http")),
        "an icon carries an id, a name and somewhere to fetch it from",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_status_types() {
    let Some(workspace) = workspace_id().await else { return };

    let statuses =
        assets(&workspace).status_types().find_status_types().send().await.expect("the workspace lists its statuses");

    assert!(
        statuses.iter().all(|status| !status.id.is_empty() && (0..=2).contains(&status.category)),
        "a status type is one of the three categories the API documents: {statuses:?}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_what_the_tenant_is_using() {
    let Some(workspace) = workspace_id().await else { return };

    let usage = assets(&workspace).usage().get_tenant_usage_info().send().await.expect("the tenant reports its usage");

    let per_schema: i64 = usage.per_schema_usage_info.iter().map(|schema| schema.object_count).sum();

    assert!(usage.total_objects_count >= 0, "an object count is not negative: {}", usage.total_objects_count);
    assert!(per_schema <= usage.total_objects_count, "the per-schema breakdown fits inside the total it belongs to");
    assert!(
        usage.per_schema_usage_info.iter().all(|schema| !schema.schema_name.is_empty()),
        "each schema in the breakdown is named",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn counts_objects_by_aql() {
    let Some(workspace) = workspace_id().await else { return };

    let count = assets(&workspace)
        .objects()
        .count_objects_by_aql(ObjectAQLTotalCountParams { ql_query: "objectType is not empty".to_owned() })
        .send()
        .await
        .expect("the workspace counts what the query matches");

    assert!(count.total_count.is_some_and(|total| total >= 0), "a count comes back and is not negative: {count:?}");
}

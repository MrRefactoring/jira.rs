use jira::cloud::CloudClient;
use jira::{Auth, Client};

use crate::harness::{cloud, require_live_env};

const DEAD_TOKEN: &str = "this-api-token-was-never-valid";

fn dead_token_client() -> CloudClient {
    let env = require_live_env();

    CloudClient::new(
        Client::builder()
            .host(env.host)
            .auth(Auth::api_token(env.email, DEAD_TOKEN))
            .build()
            .expect("a dead token still describes a usable client"),
    )
}

fn unknown_account_client() -> CloudClient {
    CloudClient::new(
        Client::builder()
            .host(require_live_env().host)
            .auth(Auth::api_token("nobody@example.invalid", "not-a-token"))
            .build()
            .expect("credentials that name nobody still describe a usable client"),
    )
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn throws_instead_of_handing_back_the_anonymous_result() {
    let error = dead_token_client()
        .projects()
        .search_projects()
        .max_results(1)
        .send()
        .await
        .expect_err("a refused credential is a failure however Jira answers it");

    assert!(error.is_auth(), "the refusal is typed as an auth failure: {error}");
    assert!(!error.is_scope(), "a dead token is not a missing scope");
    assert_eq!(
        error.status(),
        Some(200),
        "the status is the one that crossed the wire — Jira served the request as the anonymous user rather than \
         refusing it. If Atlassian ever answers 401 here the client is not wrong, but the reason this check exists \
         has gone, and that is worth knowing: {error}",
    );
    assert!(
        error.to_string().contains("x-seraph-loginreason"),
        "the message names what gave the refusal away, since the status cannot: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn throws_on_an_endpoint_that_refuses_anonymous_access_too() {
    let error = unknown_account_client()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect_err("an endpoint that answers with your own account cannot answer without one");

    assert!(error.is_auth(), "{error}");
    assert_eq!(error.status(), Some(401), "an endpoint with no anonymous answer refuses outright");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn leaves_a_working_token_alone() {
    let page = cloud()
        .projects()
        .search_projects()
        .max_results(1)
        .send()
        .await
        .expect("the live credentials read the project list");

    assert!(!page.values.is_empty(), "the site lists at least one project");

    let user = cloud().myself().get_current_user().send().await.expect("the live credentials name a user");

    assert!(
        user.account_id.as_deref().is_some_and(|id| !id.is_empty()),
        "a working token resolves to an account rather than to the anonymous user: {user:?}",
    );
    assert_eq!(user.active, Some(true), "the account the suites run as is active");
}

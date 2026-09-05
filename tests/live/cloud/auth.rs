//! Ported from jira.js/tests/live/cloud/auth.test.ts.
//!
//! A refused credential is an error, whatever status Jira dresses it in.
//!
//! The reason this suite exists rather than a unit test alone: the behaviour it pins is Jira's, not the library's.
//! Roughly a quarter of the platform's operations can be reached anonymously, and on those a dead API token does not
//! fail the request — Jira serves it as the anonymous user and reports the refusal only in `X-Seraph-LoginReason`.
//! The transport reads that header ahead of the status and turns a `200` into an [`jira::Error::Api`] of kind
//! `Auth`, which is why the status these tests assert is the one that crossed the wire rather than 401.
//!
//! Every failing case builds its own throwaway client. The shared one carries the credentials the rest of the run
//! depends on, and a suite that hands it a dead token is a suite that breaks the others.

use jira::cloud::CloudClient;
use jira::{Auth, Client};

use crate::harness::{cloud, require_live_env};

/// A token shaped like an API token and accepted by nothing.
const DEAD_TOKEN: &str = "this-api-token-was-never-valid";

/// The site's real account, with a token that was never valid.
///
/// The email is the real one deliberately: pairing a known account with a dead token is what makes Jira take the
/// anonymous fallback path rather than simply refusing an unknown user.
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

/// Credentials that name nobody at all, for the endpoints where identity is the thing being refused.
fn unknown_account_client() -> CloudClient {
    CloudClient::new(
        Client::builder()
            .host(require_live_env().host)
            .auth(Auth::api_token("nobody@example.invalid", "not-a-token"))
            .build()
            .expect("credentials that name nobody still describe a usable client"),
    )
}

/// The premise of the whole feature: a 2xx that is really a rejection, and the client refusing to pass it off as one.
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

/// The other half of the sentence: a working token is left alone.
///
/// There is no header assertion to make here, because the transport already makes it — a refusal header on any of
/// these responses would have turned them into errors ahead of the status, and every one of them succeeds.
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

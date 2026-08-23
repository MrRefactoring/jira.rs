//! The Agile DevOps modules: builds, deployments, feature flags, components, remote links, security information,
//! operations and development information.
//!
//! Eight modules in one file, deliberately. They are not eight APIs so much as one shape repeated: an app pushes
//! entities into Jira with a submit, reads one back by its own key, and deletes by key or by property. None of them
//! has a listing endpoint, and none is reachable with user credentials — they authenticate as the app that owns the
//! data, so a user token has nothing to identify.
//!
//! Written per module this would be eight near-identical assertions that a 4xx arrives. What is worth pinning is the
//! shape they share and the two consequences of it: the entity ids are the *provider's* rather than Jira's, and every
//! one of these endpoints is a write into someone's delivery pipeline data.

use jira::agile::{SubmitBuildsRequestBuilds, SubmitDeploymentsRequestDeployments, SubmitFeatureFlagsRequestFlags};

use crate::harness::agile;

/// Every read in this family is addressed by the provider's own identifiers, never by a Jira id — which is why none
/// of them can be reached with a user token, and why the refusal has to be a typed 4xx rather than a hang.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_every_read_addressed_by_a_provider_id() {
    let refusals = [
        agile()
            .builds()
            .get_build_by_key("jjs-pipeline", 1)
            .send()
            .await
            .expect_err("a user token is not the app that owns build data"),
        agile()
            .deployments()
            .get_deployment_by_key("jjs-pipeline", "jjs-env", 1)
            .send()
            .await
            .expect_err("a user token is not the app that owns deployment data"),
        agile()
            .feature_flags()
            .get_feature_flag_by_id("jjs-flag")
            .send()
            .await
            .expect_err("a user token is not the app that owns feature flag data"),
        agile()
            .devops_components()
            .get_component_by_id("jjs-comp")
            .send()
            .await
            .expect_err("a user token is not the app that owns component data"),
        agile()
            .remote_links()
            .get_remote_link_by_id("jjs-link")
            .send()
            .await
            .expect_err("a user token is not the app that owns remote link data"),
    ];

    for error in &refusals {
        assert!(
            error.status().is_some_and(|status| (400..500).contains(&status)),
            "the refusal is the caller's, not the server's: {error}",
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_submits_so_nothing_is_pushed_into_pipeline_data() {
    let builds = agile()
        .builds()
        .submit_builds(Vec::<SubmitBuildsRequestBuilds>::new())
        .send()
        .await
        .expect_err("a user token cannot submit builds");
    let deployments = agile()
        .deployments()
        .submit_deployments(Vec::<SubmitDeploymentsRequestDeployments>::new())
        .send()
        .await
        .expect_err("a user token cannot submit deployments");
    let flags = agile()
        .feature_flags()
        .submit_feature_flags(Vec::<SubmitFeatureFlagsRequestFlags>::new())
        .send()
        .await
        .expect_err("a user token cannot submit feature flags");

    for error in [&builds, &deployments, &flags] {
        assert!(error.status().is_some_and(|status| status >= 400), "{error}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_delete_by_property_variants() {
    let error = agile()
        .builds()
        .delete_builds_by_property("absent-account")
        .send()
        .await
        .expect_err("a user token cannot bulk delete build data");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_development_information_reads() {
    let error = agile()
        .development_information()
        .get_repository("jjs-repo")
        .send()
        .await
        .expect_err("a user token is not the app that owns repository data");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_security_and_operations_workspace_reads() {
    let security = agile()
        .security_information()
        .get_linked_workspaces()
        .send()
        .await
        .expect_err("a user token has no linked security workspaces");
    let operations = agile()
        .operations()
        .get_workspaces()
        .send()
        .await
        .expect_err("a user token has no linked operations workspaces");

    assert!(security.status().is_some(), "the security refusal carries a status: {security}");
    assert!(operations.status().is_some(), "the operations refusal carries a status: {operations}");
}

/// The part the library owns is not that these endpoints are unreachable — that is the token's doing — but that being
/// unreachable arrives as a status a caller can branch on.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_and_promptly_rather_than_hanging() {
    let error = agile().builds().get_build_by_key("jjs", 1).send().await.expect_err("the build does not exist here");

    assert!(error.status().is_some(), "the failure carries an HTTP status: {error}");
}

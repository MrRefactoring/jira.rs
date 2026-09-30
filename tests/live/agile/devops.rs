use jira::agile::{SubmitBuildsRequestBuilds, SubmitDeploymentsRequestDeployments, SubmitFeatureFlagsRequestFlags};

use crate::harness::agile;

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
        assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_and_promptly_rather_than_hanging() {
    let error = agile().builds().get_build_by_key("jjs", 1).send().await.expect_err("the build does not exist here");

    assert!(error.status().is_some(), "the failure carries an HTTP status: {error}");
}

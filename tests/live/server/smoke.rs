use crate::harness::server;

/// The Data Center surface reaches a self-hosted instance at all.
///
/// Everything the Cloud suites prove about the transport is proven there; what is genuinely different here is that a
/// self-hosted instance is its own host with no gateway, answers on `/rest/api/2` rather than `/rest/api/3`, and
/// authenticates a local account rather than an Atlassian one.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_instance_it_is_talking_to() {
    // Read unmodelled on purpose. Atlassian's Data Center specification declares `ServerInfo` with no properties at
    // all, so the generated type is an empty struct — a faithful rendering of a document that describes nothing. The
    // gap belongs in the generator's patches; asserting against the body is what proves it is a gap rather than a
    // limit of the client.
    let info = server().server_info().get_server_info().send_raw().await.expect("the instance describes itself");

    assert!(info["version"].as_str().is_some_and(|version| !version.is_empty()), "{info}");
    assert_eq!(info["deploymentType"].as_str(), Some("Server"), "the rig is a self-hosted deployment");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_local_account_the_credentials_belong_to() {
    let myself = server().myself().get_current_user().send().await.expect("the instance knows the caller");

    assert!(myself.name.is_some_and(|name| !name.is_empty()), "a Data Center user is addressed by name, not by id");
    assert!(myself.active.unwrap_or(false), "the credentials belong to an active user");
}

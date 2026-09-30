use crate::harness::server;

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_instance_it_is_talking_to() {
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

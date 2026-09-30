use jira::admin::{AdminClient, MultiDirectoryUserSearchRequest};
use jira::user_management::UserManagementClient;

use crate::harness::{admin_key_client, has_admin_env, org_id, user_management};

fn keyed_clients() -> Option<(UserManagementClient, AdminClient)> {
    if has_admin_env() {
        return Some((
            UserManagementClient::new(admin_key_client().clone()),
            AdminClient::new(admin_key_client().clone()),
        ));
    }

    let config = user_management()
        .manage()
        .get_management_permissions("unclaimed-account".to_owned())
        .config()
        .expect("the request is well formed");

    assert_eq!(
        config.url, "/users/unclaimed-account/manage",
        "user management addresses the account itself, not a path any site serves",
    );

    None
}

async fn some_account(admin: &AdminClient, org: &str) -> String {
    let directories =
        admin.directory().get_directories_for_org(org).send().await.expect("the organization lists its directories");

    let directory = directories
        .data
        .unwrap_or_default()
        .into_iter()
        .next()
        .and_then(|directory| directory.directory_id)
        .expect("the organization has a directory to address");

    let page = admin
        .users()
        .search_directory_users(org, &directory)
        .multi_directory_user_search_request(MultiDirectoryUserSearchRequest {
            limit: Some(1),
            ..MultiDirectoryUserSearchRequest::default()
        })
        .send()
        .await
        .expect("the directory answers a user search");

    page.data.into_iter().next().and_then(|user| user.account_id).expect("the directory holds at least one account")
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_scoped_organization_key_and_says_which_scope_it_wanted() {
    let org = org_id().await;
    let Some((users, admin)) = keyed_clients() else { return };
    let account_id = some_account(&admin, &org).await;

    let error = users
        .manage()
        .get_management_permissions(account_id)
        .send()
        .await
        .expect_err("the scoped key was accepted — the organization or the key changed");

    assert!(error.is_forbidden(), "the refusal is about rights rather than credentials: {error}");

    let body = error.body().expect("a refusal carries Atlassian's error payload").to_string();

    assert!(body.contains("insufficient"), "the refusal says what was insufficient: {body}");
    assert!(body.contains("manage:org"), "the refusal names the scope it would have accepted: {body}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_read_of_the_profile_on_the_same_grounds() {
    let org = org_id().await;
    let Some((users, admin)) = keyed_clients() else { return };
    let account_id = some_account(&admin, &org).await;

    let error =
        users.profile().get_profile(account_id).send().await.expect_err("the profile is not readable with this key");

    assert!(error.is_forbidden(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_read_of_the_api_tokens_on_the_same_grounds() {
    let org = org_id().await;
    let Some((users, admin)) = keyed_clients() else { return };
    let account_id = some_account(&admin, &org).await;

    let error = users
        .api_tokens()
        .get_api_tokens(account_id)
        .send()
        .await
        .expect_err("the API tokens are not readable with this key");

    assert!(error.is_forbidden(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn has_no_managed_account_to_act_on() {
    let org = org_id().await;
    let Some((_, admin)) = keyed_clients() else { return };

    let page = admin.users().get_users(&org).send().await.expect("the organization lists its managed accounts");
    let managed = page.data.expect("the envelope carries a listing, empty or not");

    assert!(managed.is_empty(), "nothing is manageable here, got {} accounts", managed.len());
}

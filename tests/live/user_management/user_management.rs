//! The user management API.
//!
//! What this suite can prove is narrower than the surface, and the reason is measured rather than assumed. Two things
//! stand in the way, and both belong to the organization rather than to the library:
//!
//! 1. **A scoped API key is refused outright.** Every operation answers `403 forbidden.insufficientScope` and names
//!    `manage:org` among the scopes it would accept — a scope the key creation flow does not offer. The
//!    `read:*:admin` scopes that reach the organization API reach nothing here.
//! 2. **Nothing on this organization is manageable.** Its one account reports `claimStatus: unmanaged`, and the
//!    organization's managed-account listing returns none. These operations act on accounts whose email domain the
//!    organization has claimed and verified; with no claimed domain there is no subject.
//!
//! So the suite pins the refusal and its shape, which is worth pinning: a caller who reaches for this API with the key
//! that works everywhere else gets a typed forbidden error naming the scope, not a silent empty answer. The rest
//! stands down visibly. It becomes a real suite the day the organization claims a domain and an unscoped key exists.

use jira::admin::{AdminClient, MultiDirectoryUserSearchRequest};
use jira::user_management::UserManagementClient;

use crate::harness::{admin_key_client, has_admin_env, org_id, user_management};

/// The two clients the suite runs on, or `None` when the organization API key both need is absent.
///
/// CI has none, and a site API token does not substitute: these operations answer on `api.atlassian.com` and address
/// an account rather than a site. Standing down still pins that address, so an absent key never leaves a test
/// asserting nothing.
fn keyed_clients() -> Option<(UserManagementClient, AdminClient)> {
    if has_admin_env() {
        return Some((UserManagementClient::new(admin_key_client()), AdminClient::new(admin_key_client())));
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

/// One account of the organization, which is the subject every operation here needs.
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

    // The other half of why this suite reads rather than writes: the organization manages nobody. Pinned so that the
    // day a domain is claimed, this test fails and says the suite can be widened.
    let page = admin.users().get_users(&org).send().await.expect("the organization lists its managed accounts");
    let managed = page.data.expect("the envelope carries a listing, empty or not");

    assert!(managed.is_empty(), "nothing is manageable here, got {} accounts", managed.len());
}

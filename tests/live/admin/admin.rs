//! The organization administration API.
//!
//! Read-only throughout, and not for want of coverage: every write here acts on a real organization's users, groups
//! and policies, and there is no fixture to create and throw away as the project suites do. The key the suite runs on
//! holds only the `read:` scopes, so a write would be refused rather than silently succeed.
//!
//! Every test stands down when no organization API key is configured — CI has none, and a site API token does not
//! substitute: these operations answer on `api.atlassian.com` and address the organization. Standing down still pins
//! that address, so an absent key never leaves a test asserting nothing.

use jira::admin::{AdminClient, MultiDirectoryGroupSearchRequest, MultiDirectoryUserSearchRequest, OrgModelType};

use crate::harness::{admin_key_client, admin_surface, has_admin_env, org_id};

/// The administration client, or `None` when the organization API key the whole surface needs is absent.
fn administration(org: &str) -> Option<AdminClient> {
    if has_admin_env() {
        return Some(AdminClient::new(admin_key_client()));
    }

    let config = admin_surface().orgs().get_org_by_id(org).config().expect("the request is well formed");

    assert_eq!(
        config.url,
        format!("/admin/v1/orgs/{org}"),
        "the administration surface addresses the organization, which is why a site token cannot reach it",
    );

    None
}

/// The first directory of the organization, which is what every user and group operation is addressed to.
async fn first_directory(admin: &AdminClient, org: &str) -> String {
    let directories =
        admin.directory().get_directories_for_org(org).send().await.expect("the organization lists its directories");

    let directory = directories
        .data
        .unwrap_or_default()
        .into_iter()
        .next()
        .expect("the organization has a directory to address");

    directory.directory_id.expect("a directory is named by an id")
}

/// One account from the directory, for the operations that need a subject.
async fn first_account(admin: &AdminClient, org: &str, directory: &str) -> String {
    let page = admin
        .users()
        .search_directory_users(org, directory)
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
async fn reads_the_organization_it_is_pointed_at() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };

    let organization = admin.orgs().get_org_by_id(&org).send().await.expect("the organization reads back by id");
    let data = organization.data.expect("the envelope carries the organization");

    assert_eq!(data.id, org, "the organization answering is the one that was asked for");
    assert_eq!(data.r#type, OrgModelType::Orgs);
    assert!(data.attributes.name.is_some_and(|name| !name.is_empty()), "an organization carries a name");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_no_organizations_at_all_because_the_key_is_scoped_to_one() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };

    // Not a defect and not an empty tenant: a key created with scopes belongs to a single organization, and the
    // listing endpoint answers 200 with nothing while the direct read above works. Pinned so that a future empty
    // result is read as this rather than as a broken credential.
    let page = admin.orgs().get_orgs().send().await.expect("the listing answers");
    let organizations = page.data.expect("the envelope carries a listing, empty or not");

    assert!(organizations.is_empty(), "a scoped key lists none, got {}", organizations.len());
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_a_user_in_the_directory_and_reads_it_back_by_id() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };
    let directory = first_directory(&admin, &org).await;

    let page = admin
        .users()
        .search_directory_users(&org, &directory)
        .multi_directory_user_search_request(MultiDirectoryUserSearchRequest {
            limit: Some(5),
            ..MultiDirectoryUserSearchRequest::default()
        })
        .send()
        .await
        .expect("the directory answers a user search");

    assert!(!page.data.is_empty(), "a directory with no users at all is not the organization under test");
    assert!(page.data.len() <= 5, "a page holds no more than the limit asked for");

    let account_id = page.data[0].account_id.clone().expect("a directory user is named by an account id");
    let details = admin
        .users()
        .get_directory_user_details(&org, &directory, &account_id)
        .send()
        .await
        .expect("the user reads back by account id");

    assert_eq!(
        details.data.and_then(|data| data.account_id).as_deref(),
        Some(account_id.as_str()),
        "the detail read answers about the account it was asked for",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_a_users_role_assignments() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };
    let directory = first_directory(&admin, &org).await;
    let account_id = first_account(&admin, &org, &directory).await;

    let page = admin
        .users()
        .get_user_role_assignments(&org, &directory, &account_id)
        .send()
        .await
        .expect("the user's role assignments read back");

    let assignments = page.data.expect("the envelope carries the assignments, empty or not");

    assert!(
        assignments.iter().all(|assignment| assignment.resource_id.is_some()),
        "a role assignment names the resource it is on",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_groups_and_counts_them() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };
    let directory = first_directory(&admin, &org).await;

    let page = admin
        .groups()
        .search_directory_groups(&org, &directory)
        .multi_directory_group_search_request(MultiDirectoryGroupSearchRequest {
            limit: Some(10),
            ..MultiDirectoryGroupSearchRequest::default()
        })
        .send()
        .await
        .expect("the directory answers a group search");

    assert!(!page.data.is_empty(), "an organization always has the groups its products need");
    assert_eq!(
        page.data[0].directory_id.as_deref(),
        Some(directory.as_str()),
        "a group comes back attributed to the directory it was searched in",
    );

    let count =
        admin.groups().get_groups_count(&org, &directory).send().await.expect("the directory counts its groups");

    assert!(
        count.count.is_some_and(|total| total >= page.data.len() as i64),
        "the count covers at least the page that was just read: {:?}",
        count.count,
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_policies_and_reads_one_back() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };

    let page = admin.policies().get_policies(&org).send().await.expect("the organization lists its policies");
    let policies = page.data.expect("the envelope carries the policies");

    assert!(!policies.is_empty(), "an organization carries at least its default policy");

    let first = &policies[0];
    let one = admin.policies().get_policy_by_id(&org, &first.id).send().await.expect("the policy reads back by id");

    assert_eq!(one.data.expect("the envelope carries the policy").id, first.id);
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_for_the_audit_trail_empty_or_not() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };

    // A quiet organization has no events, so the assertion is on the envelope rather than on its contents.
    let events = admin.events().get_events(&org).send().await.expect("the organization answers for its audit trail");
    let recorded = events.data.expect("the envelope carries the events, empty or not");

    assert!(recorded.iter().all(|event| !event.id.is_empty()), "an audit event is named by an id");

    let actions = admin.events().get_event_actions(&org).send().await.expect("the audit trail names its actions");

    assert!(actions.data.is_some(), "the envelope carries the actions the trail can record");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_products_the_organization_owns() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };

    let page = admin.workspaces().query_workspaces(&org).send().await.expect("the organization lists its workspaces");
    let workspaces = page.data.expect("the envelope carries the workspaces");

    assert!(!workspaces.is_empty(), "the organization owns the site these suites run against");
    assert!(
        workspaces.iter().all(|workspace| workspace.id.as_deref().is_some_and(|id| !id.is_empty())),
        "a workspace is named by an id",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_what_the_key_is_not_entitled_to_as_a_typed_error() {
    let org = org_id().await;
    let Some(admin) = administration(&org) else { return };
    let directory = first_directory(&admin, &org).await;
    let account_id = first_account(&admin, &org, &directory).await;

    // Last active dates are a paid feature, and the read-scoped key is refused. What matters is that the refusal
    // arrives typed rather than as a resolved empty answer.
    match admin.users().get_user_last_active_dates(&org, &account_id).send().await {
        Ok(activity) => assert!(activity.data.is_some(), "an entitled answer carries the product activity envelope"),
        Err(error) => assert!(error.is_forbidden(), "the refusal names the rights rather than the request: {error}"),
    }
}

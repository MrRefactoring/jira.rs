use jira::cloud::{DashboardUserAccountType, GetUserRequestExpand, GetUserRequestExpandValue, UserColumnRequestBody};

use crate::harness::{ResourceTracker, await_readable, cloud};

/// The account the token authenticates as.
///
/// Everything asserted here is about that one account. Creating or removing a user is an identity operation with
/// billing consequences and no clean undo, so neither is aimed at anything real — they are pinned through their
/// error channel alone.
async fn current_account_id() -> String {
    cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site names the authenticating account")
        .account_id
        .expect("the authenticating account carries an id")
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_the_authenticating_account_by_account_id() {
    let account_id = current_account_id().await;

    let user = cloud().users().get_user().account_id(account_id.as_str()).send().await.expect("the account reads back");

    assert_eq!(user.account_id.as_deref(), Some(account_id.as_str()));
    assert_eq!(user.active, Some(true), "the account the suite authenticates as is an active one");
    assert_eq!(user.account_type, Some(DashboardUserAccountType::Atlassian));

    let link = user.self_.expect("a user carries a link to itself");

    assert!(link.starts_with("https://"), "a self link is absolute: {link}");
}

/// Cloud hides personal data by default, so `emailAddress` and `displayName` may legitimately be absent. Code that
/// assumes otherwise works on one tenant and breaks on the next.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn treats_personal_data_as_optional_because_privacy_settings_make_it_so() {
    let account_id = current_account_id().await;

    let user = cloud().users().get_user().account_id(account_id.as_str()).send().await.expect("the account reads back");

    if let Some(email) = &user.email_address {
        assert!(email.contains('@'), "an email address that is shown is an address: {email}");
    }

    if let Some(name) = &user.display_name {
        assert!(!name.trim().is_empty(), "a display name that is shown is not blank");
    }

    let avatar = user.avatar_urls.and_then(|urls| urls.n48x48).expect("an avatar is published whatever the privacy");

    assert!(avatar.starts_with("https://"), "an avatar is an absolute URL: {avatar}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn expands_groups_only_when_asked() {
    let account_id = current_account_id().await;

    let plain =
        cloud().users().get_user().account_id(account_id.as_str()).send().await.expect("the account reads back");

    let expanded = cloud()
        .users()
        .get_user()
        .account_id(account_id.as_str())
        .expand(GetUserRequestExpand::One(GetUserRequestExpandValue::Groups))
        .send()
        .await
        .expect("the groups expansion is accepted");

    let unexpanded = plain.groups.and_then(|wrapper| wrapper.items).unwrap_or_default();

    assert!(unexpanded.is_empty(), "groups stay unlisted until the expansion asks for them");

    let wrapper = expanded.groups.expect("the expansion carries a groups wrapper");
    let size = wrapper.size.expect("the wrapper reports how many groups it counted");
    let items = wrapper.items.unwrap_or_default();

    assert_eq!(items.len(), usize::try_from(size).expect("a size is not negative"), "the wrapper lists what it counts");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_groups_the_account_belongs_to() {
    let account_id = current_account_id().await;

    let groups = cloud().users().get_user_groups(account_id).send().await.expect("the account's groups read back");

    // Product access on Cloud is granted to groups, so a licensed account is in at least one.
    assert!(!groups.is_empty(), "an account with product access belongs to a group");

    for group in &groups {
        assert!(group.name.as_deref().is_some_and(|name| !name.is_empty()), "a group carries a name: {group:?}");
        assert!(group.group_id.as_deref().is_some_and(|id| !id.is_empty()), "a group carries an id: {group:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_site_user_listing() {
    let first = cloud().users().get_all_users().max_results(2).send().await.expect("the site lists its users");

    assert!(first.len() <= 2, "maxResults caps the page, got {} users", first.len());

    for user in &first {
        assert!(user.account_id.as_deref().is_some_and(|id| !id.is_empty()), "a listed user carries an id");
        assert!(user.account_type.is_some(), "a listed user carries an account type: {user:?}");
    }

    let offset =
        cloud().users().get_all_users().max_results(2).start_at(2).send().await.expect("the listing accepts an offset");

    if first.len() == 2 && !offset.is_empty() {
        let first_ids: Vec<_> = first.iter().map(|user| user.account_id.clone()).collect();
        let offset_ids: Vec<_> = offset.iter().map(|user| user.account_id.clone()).collect();

        assert_ne!(first_ids, offset_ids, "startAt moves the window rather than repeating the first page");
    }
}

/// The email endpoint is reserved for approved apps: user credentials are refused whatever the account, and the
/// refusal is a 400 about the credential rather than a 403 about rights.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_to_report_an_email_to_user_credentials_at_all() {
    let account_id = current_account_id().await;

    let error = cloud()
        .users()
        .get_user_email(account_id)
        .send()
        .await
        .expect_err("user credentials cannot read an email address");

    assert_eq!(error.status(), Some(400), "{error}");
    assert!(!error.is_forbidden(), "the refusal is about the credential type, not about rights");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_account_as_a_typed_error() {
    let error = cloud()
        .users()
        .get_user()
        .account_id("no-such-account-id")
        .send()
        .await
        .expect_err("an account that does not exist cannot be read");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path_without_ever_aiming_it_at_a_real_account() {
    let error = cloud()
        .users()
        .remove_user("no-such-account-id")
        .send()
        .await
        .expect_err("an account that does not exist cannot be removed");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The one write in this suite, and it belongs to the authenticating account alone: the issue navigator columns this
/// account sees. The reset is deferred as well as called, so a failed assertion still leaves the account as it was.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn sets_the_calling_accounts_issue_navigator_columns_then_resets_them() {
    let mut tracker = ResourceTracker::new();

    let before =
        await_readable("the calling account's columns read back", || cloud().users().get_user_default_columns().send())
            .await;

    let before_values: Vec<_> = before.iter().map(|column| column.value.clone()).collect();

    tracker.defer(|| async { cloud().users().reset_user_columns().send().await });

    cloud()
        .users()
        .set_user_columns(UserColumnRequestBody { columns: Some(vec!["summary".to_owned(), "status".to_owned()]) })
        .send()
        .await
        .expect("the calling account's columns can be set");

    let columns =
        await_readable("the new columns read back", || cloud().users().get_user_default_columns().send()).await;
    let values: Vec<_> = columns.iter().filter_map(|column| column.value.clone()).collect();

    assert_eq!(values, ["summary", "status"], "the columns set are the columns returned, in order");

    cloud().users().reset_user_columns().send().await.expect("the columns can be reset");

    let reset =
        await_readable("the reset columns read back", || cloud().users().get_user_default_columns().send()).await;
    let reset_values: Vec<_> = reset.iter().map(|column| column.value.clone()).collect();

    assert_eq!(reset_values, before_values, "the reset puts the site default back");

    tracker.cleanup().await;
}

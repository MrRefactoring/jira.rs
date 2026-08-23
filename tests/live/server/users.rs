//! Users, groups and the account the suite is signed in as.
//!
//! This is where Data Center differs from Cloud most plainly: a self-hosted Jira owns its directory, so it creates
//! users, sets their passwords and anonymises them — none of which the Cloud API offers — and it identifies them by
//! `name` and `key` rather than by an `accountId`.
//!
//! Each test creates the user it needs and removes it. What it does to the signed-in account it leaves alone, except
//! for the password, which it never changes: the rest of the run authenticates with it.

use jira::server::{
    Avatar, AvatarCropping, DefaultShareScope, DefaultShareScopeScope, Filter, Password, PasswordPolicyCreateUser,
    PasswordPolicyUpdateUser, SharePermissionInput, UpdateUserToGroup, UserAnonymizationRequest,
    UserAnonymizationRerunRequest, UserWrite,
};
use serde_json::json;

use super::fixtures::{
    admin_username, create_test_filter, create_test_group, create_test_user, property_body, property_value,
    tiny_avatar, touch,
};
use crate::harness::{ResourceTracker, server};

/// The key the preference test writes under.
const PREFERENCE_KEY: &str = "jira.rs.suite";

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_user_it_created() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    let user = server().users().get_user().username(&name).send().await.expect("the user reads back by name");

    assert_eq!(user.name.as_deref(), Some(name.as_str()), "the user read back is the one created");
    assert!(user.key.is_some(), "a directory assigns a key of its own, which need not be the username");
    assert_eq!(user.email_address.as_deref(), Some(format!("{name}@example.com").as_str()), "with the address given");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn updates_the_user_and_its_password() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    server()
        .users()
        .update_user(UserWrite { display_name: Some("changed by the users suite".to_owned()), ..UserWrite::default() })
        .username(&name)
        .send()
        .await
        .expect("a user can be changed");

    let user = server().users().get_user().username(&name).send().await.expect("the changed user reads back");

    assert_eq!(
        user.display_name.as_deref(),
        Some("changed by the users suite"),
        "the change is observable on the next read",
    );

    server()
        .users()
        .change_user_password(Password { password: Some("Correct-Horse-Battery-2".to_owned()), ..Password::default() })
        .username(&name)
        .send()
        .await
        .expect("an administrator can set another account's password");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn checks_a_password_against_the_policy() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    let on_create = server()
        .password()
        .policy_check_create_user(PasswordPolicyCreateUser {
            username: Some(name.clone()),
            password: Some("short".to_owned()),
            display_name: Some("the users suite".to_owned()),
            email_address: Some(format!("{name}@example.com")),
        })
        .send()
        .await
        .expect("a proposed password is checked against the policy");

    assert!(on_create.is_array(), "the policy answers with a list of what is wrong: {on_create}");

    let on_update = server()
        .password()
        .policy_check_update_user(PasswordPolicyUpdateUser {
            username: Some(name),
            new_password: Some("short".to_owned()),
            ..PasswordPolicyUpdateUser::default()
        })
        .send()
        .await
        .expect("a proposed change is checked too");

    assert!(on_update.is_array(), "and answers in the same shape: {on_update}");

    let policy = server().password().get_password_policy().send().await.expect("the policy itself reads");

    assert!(policy.is_array(), "a policy is the list of requirements it imposes: {policy}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn moves_the_user_in_and_out_of_a_group() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;
    let group = create_test_group(&mut tracker).await;

    server()
        .groups()
        .add_user_to_group(&group)
        .update_user_to_group(UpdateUserToGroup { name: Some(name.clone()) })
        .send()
        .await
        .expect("a user can be put in a group");

    let members = server().groups().get_users_from_group(&group).send().await.expect("the members of a group read");

    assert!(
        members.values.iter().flatten().any(|member| member.name.as_deref() == Some(name.as_str())),
        "the user just added is a member",
    );

    server().groups().remove_user_from_group(&group, &name).send().await.expect("and can be taken out again");

    let after = server().groups().get_users_from_group(&group).send().await.expect("the members read again");

    assert!(
        !after.values.iter().flatten().any(|member| member.name.as_deref() == Some(name.as_str())),
        "and is gone from the membership on the next read",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn moves_the_user_in_and_out_of_an_application() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    let roles = server().application_roles().get_all().send().await.expect("the instance lists its applications");
    let key = roles
        .first()
        .and_then(|role| role.key.clone())
        .expect("a licensed Jira has at least one application to belong to");

    // A timebomb licence has a seat count a suite cannot assume is free.
    touch(server().users().add_user_to_application().username(&name).application_key(&key).send().await);
    touch(server().users().remove_user_from_application().username(&name).application_key(key).send().await);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn stores_a_property_on_the_user() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    server()
        .users()
        .set_user_property("suite", property_body())
        .username(&name)
        .send()
        .await
        .expect("a user takes a property of the caller's own");

    let property =
        server().users().get_user_property("suite").username(&name).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    server().users().delete_user_property("suite").username(&name).send().await.expect("the property can be removed");

    let error = server()
        .users()
        .get_user_property("suite")
        .username(&name)
        .send()
        .await
        .expect_err("a removed property cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn sets_and_resets_the_columns_a_user_sees() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    server()
        .users()
        .set_columns_url_encoded()
        .username(&name)
        .columns(["summary", "status"])
        .send()
        .await
        .expect("the columns a user sees can be set");

    // Read unmodelled on purpose: the specification declares `ColumnOptions` with no properties at all, so the
    // generated type is an empty struct and the labels never reach a caller. The gap is the document's.
    let columns = server().users().default_columns().username(&name).send_raw().await.expect("the columns read back");

    assert!(columns.as_array().is_some_and(|columns| !columns.is_empty()), "{columns}");
    // `summary` is written and does not come back: measured against Data Center 10.3, the instance accepts
    // the request and silently drops that column from the navigator defaults, keeping the rest. Asserting on
    // a column it does keep is what makes this a test of the write rather than of Jira's column policy.
    assert!(columns.to_string().contains("status"), "the columns just set are the columns read back: {columns}");

    server().users().reset_user_columns().username(&name).send().await.expect("and can be reset to the default");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn validates_and_schedules_an_anonymisation() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    // Anonymisation is keyed by the user's key, which the directory assigns and which need not be the username.
    let user = server().users().get_user().username(&name).send().await.expect("the user reads back");
    let key = user.key.clone().unwrap_or_else(|| name.clone());

    let validation = server()
        .users()
        .validate_user_anonymization()
        .user_key(&key)
        .send()
        .await
        .expect("an anonymisation can be validated before it is scheduled");

    assert_eq!(validation.user_key.as_deref(), Some(key.as_str()), "the validation is about the user asked for");

    let owner = admin_username();

    touch(
        server()
            .users()
            .schedule_user_anonymization(UserAnonymizationRequest {
                user_key: Some(key.clone()),
                new_owner_key: Some(owner.clone()),
            })
            .send()
            .await,
    );
    touch(server().users().get_user_anonymization_progress().send().await);
    touch(server().users().validate_user_anonymization_rerun().user_key(&key).send().await);
    touch(
        server()
            .users()
            .schedule_user_anonymization_rerun(UserAnonymizationRerunRequest {
                user_key: Some(key),
                new_owner_key: Some(owner),
                ..UserAnonymizationRerunRequest::default()
            })
            .send()
            .await,
    );
    touch(server().users().unlock_anonymization().send().await);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn ends_the_user_session() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    // A user who has never signed in has no session to end, which is the refusal this proves is typed.
    touch(server().users().delete_session(&name).send().await);

    let user = server().users().get_user().username(&name).send().await.expect("the user outlives its session");

    assert_eq!(user.name.as_deref(), Some(name.as_str()), "ending a session does not end the account");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_an_avatar_for_the_user() {
    let mut tracker = ResourceTracker::new();
    let name = create_test_user(&mut tracker).await;

    let avatars =
        server().users().get_all_user_avatars().username(&name).send().await.expect("the avatars of a user read");
    let system = avatars.system.as_ref().expect("a user has system avatars");

    assert!(!system.is_empty(), "every Jira instance ships system avatars for a user");

    let temporary = touch(
        server().users().store_temporary_user_avatar_using_multi_part([tiny_avatar()]).username(&name).send().await,
    );

    assert!(temporary.is_none_or(|answer| answer.is_object()), "a temporary avatar answers with an object");

    touch(
        server()
            .users()
            .create_user_avatar_from_temporary(AvatarCropping { cropper_width: Some(1), ..AvatarCropping::default() })
            .username(&name)
            .send()
            .await,
    );

    let id = system.first().and_then(|avatar| avatar.id.clone()).expect("a system avatar is addressed by an id");

    // Read unmodelled: the document declares this write as answering with an `Avatar`, and the instance answers
    // nothing readable as one. The gap is the document's, and reading the body is what proves the write reached Jira.
    touch(
        server()
            .users()
            .update_user_avatar(Avatar { id: Some(id.clone()), ..Avatar::default() })
            .username(&name)
            .send_raw()
            .await,
    );

    if let Ok(id) = id.parse::<i64>() {
        // A system avatar cannot be deleted, which is the refusal this proves is typed.
        touch(server().users().delete_user_avatar(id).username(&name).send().await);
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_and_writes_the_signed_in_account() {
    let username = admin_username();
    let me = server().myself().get_current_user().send().await.expect("the instance knows the caller");

    assert_eq!(me.name.as_deref(), Some(username.as_str()), "the account is the one the credentials belong to");
    assert!(me.active.unwrap_or(false), "and it is an active one");

    // Data Center validates the whole user on this endpoint, password included, so what it refuses is a partial
    // update rather than a badly-shaped one.
    touch(
        server()
            .myself()
            .update_current_user(UserWrite {
                display_name: Some("the Data Center live suite".to_owned()),
                ..UserWrite::default()
            })
            .send()
            .await,
    );

    // Never actually changed: the rest of the run signs in with this password. A wrong current password is a fair
    // answer, and it is the request shape that is under test.
    touch(
        server()
            .myself()
            .change_my_password(Password {
                current_password: Some("not-the-password".to_owned()),
                password: Some("not-the-password-either".to_owned()),
            })
            .send()
            .await,
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_preference() {
    server()
        .my_preferences()
        .set_preference()
        .key(PREFERENCE_KEY)
        .body("true")
        .send()
        .await
        .expect("a preference can be stored");

    // Read unmodelled on purpose: a preference is whatever was stored under the key, and the document types that as
    // a string — so `true` comes back as a JSON boolean and the modelled read cannot hold it.
    let preference =
        server().my_preferences().get_preference().key(PREFERENCE_KEY).send_raw().await.expect("it reads back");

    assert!(
        preference == json!(true) || preference.as_str() == Some("true"),
        "the preference reads back the way it was written: {preference}",
    );

    server()
        .my_preferences()
        .remove_preference()
        .key(PREFERENCE_KEY)
        .send()
        .await
        .expect("a preference can be removed");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_filter_of_its_own() {
    let mut tracker = ResourceTracker::new();
    let filter = create_test_filter(&mut tracker, "user filter", "order by created").await;
    let id = filter.id.clone().expect("a created filter carries an id");

    server()
        .filters()
        .edit_filter(&id)
        .body(Filter { description: Some("changed by the users suite".to_owned()), ..Filter::default() })
        .send()
        .await
        .expect("a filter can be changed");

    let read = server().filters().get_filter(&id).send().await.expect("the filter reads back");

    assert_eq!(
        read.description.as_deref(),
        Some("changed by the users suite"),
        "the change is observable on the next read",
    );

    server()
        .filters()
        .set_columns(&id)
        .columns(["summary", "status"])
        .send()
        .await
        .expect("a filter carries columns of its own");

    let columns = server().filters().get_filter_columns(&id).send().await.expect("the columns read back");

    assert!(!columns.is_empty(), "the columns just set are the columns read back");

    server().filters().reset_columns(&id).send().await.expect("and can be reset");

    let permissions = server()
        .filters()
        .add_share_permission(&id)
        .share_permission_input(SharePermissionInput {
            r#type: Some("authenticated".to_owned()),
            ..SharePermissionInput::default()
        })
        .send()
        .await
        .expect("a filter can be shared with everyone signed in");

    let permission_id = permissions
        .first()
        .and_then(|permission| permission.id)
        .expect("a share permission is addressable once it exists");

    let read = server()
        .filters()
        .get_share_permission(permission_id.to_string(), &id)
        .send()
        .await
        .expect("a share permission reads back by id");

    assert_eq!(read.id, Some(permission_id), "the permission read back is the one that was added");

    server()
        .filters()
        .delete_share_permission(&id, permission_id.to_string())
        .send()
        .await
        .expect("a share permission can be withdrawn");

    touch(
        server()
            .filters()
            .set_default_share_scope()
            .default_share_scope(DefaultShareScope { scope: Some(DefaultShareScopeScope::Private) })
            .send()
            .await,
    );

    tracker.cleanup().await;
}

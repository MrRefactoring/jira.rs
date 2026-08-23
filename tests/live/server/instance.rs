//! The instance-level endpoints: the ones an administrator reaches rather than a project member.
//!
//! Much of what is here cannot succeed on a single unclustered node — a cluster with no nodes, an upgrade that is not
//! pending, an index snapshot with nowhere to write. What each call proves is that the request serialises and that
//! whatever comes back matches the schema; Jira refusing on its own terms is a correct answer, and `touch` accepts it
//! while still insisting the refusal carries a status.

use jira::server::{
    AppMonitoringRestEntity, ApplicationPropertyValue, AuthParams, AvatarCropping, IpdMonitoringRestEntity,
    ReadOnlyModeUpdateRequest, TerminologyRequest,
};

use super::fixtures::{business_project, create_task, tiny_avatar, touch};
use crate::harness::{ResourceTracker, require_server_env, server};

/// The write is asserted rather than touched, which is the whole point of it being here.
///
/// The document declares this operation with a path parameter and no request body, so a generated call can be written
/// that sends none — and Jira answers 400, which `touch` would accept as one of the refusals a single node is
/// entitled to make. Reading the value out of the response is what proves the body arrived.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_and_writes_an_application_property() {
    let properties = server()
        .application_properties()
        .get_application_properties()
        .key_filter("jira.clone.prefix")
        .send()
        .await
        .expect("the instance lists its application properties");

    let property = properties.first().expect("the key filter matches the clone prefix");
    let id = property.id.clone().expect("an application property is addressed by its id");

    assert_eq!(id, "jira.clone.prefix", "the filter narrowed the listing to the property asked for");

    let written = server()
        .application_properties()
        .set_property_via_restful_table(
            id.clone(),
            ApplicationPropertyValue { id: Some(id), value: Some("DUPLICATE - ".to_owned()) },
        )
        .send()
        .await
        .expect("an application property can be written");

    assert_eq!(written.value.as_deref(), Some("DUPLICATE - "), "the value the body carried is the value that stuck");

    let advanced =
        server().application_properties().get_advanced_settings().send().await.expect("the advanced settings read");

    assert!(advanced.iter().all(|setting| setting.id.is_some()), "every advanced setting is addressable by an id");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_and_writes_an_application_role() {
    let roles = server().application_roles().get_all().send().await.expect("the instance lists its application roles");
    let role = roles.first().expect("a licensed Jira has at least one application role").clone();
    let key = role.key.clone().expect("an application role is addressed by its key");

    let read = server()
        .application_roles()
        .get_application_role(key.clone())
        .send()
        .await
        .expect("an application role reads back by key");

    assert_eq!(read.key.as_deref(), Some(key.as_str()), "the role read back is the one asked for");

    // Both writes want an `If-Match` a caller cannot know, so a refusal is the expected answer and the shape of the
    // request is what is under test.
    touch(server().application_roles().update_application_role(key).body(role.clone()).send().await);
    touch(server().application_roles().put_bulk().application_role(role).send().await);
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn sets_the_base_url_and_the_default_columns() {
    let env = require_server_env();

    touch(server().jira_settings().set_base_url().body(env.host).send().await);

    server()
        .jira_settings()
        .set_issue_navigator_default_columns_form()
        .columns(["summary", "status"])
        .send()
        .await
        .expect("the default columns can be set");

    // Read unmodelled on purpose: the Data Center specification declares `ColumnOptions` with no properties at all,
    // so the generated type is an empty struct and the labels never reach a caller. The gap is the document's.
    let columns = server()
        .jira_settings()
        .get_issue_navigator_default_columns()
        .send_raw()
        .await
        .expect("the default columns read back");

    assert!(columns.as_array().is_some_and(|columns| !columns.is_empty()), "{columns}");
    // `summary` is written and does not come back: measured against Data Center 10.3, the instance accepts
    // the request and silently drops that column from the navigator defaults, keeping the rest. Asserting on
    // a column it does keep is what makes this a test of the write rather than of Jira's column policy.
    assert!(columns.to_string().contains("status"), "the columns just set are the columns read back: {columns}");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn renames_a_term_and_puts_it_back() {
    let entries = server().terminology().get_all_terminology_entries().send().await.expect("the terms read");

    assert!(entries.iter().all(|entry| entry.original_name.is_some()), "every entry names the term it renames");

    let entry = entries.first().expect("a Jira instance ships terminology entries").clone();
    let original = entry.original_name.clone().expect("an entry is addressed by the original term");

    touch(
        server()
            .terminology()
            .set_terminology_entries(TerminologyRequest {
                original_name: Some(original.clone()),
                new_name: entry.new_name.clone().or_else(|| Some(original.clone())),
                new_name_plural: entry.new_name_plural.clone().or_else(|| Some(original.clone())),
            })
            .send()
            .await,
    );

    let read =
        server().terminology().get_terminology_entry(original.clone()).send().await.expect("a single term reads back");

    assert_eq!(read.original_name.as_deref(), Some(original.as_str()), "the term is the one that was written");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn validates_a_licence() {
    let result = server().license_validator().validate("not-a-licence").send().await.expect("a licence is validated");

    assert!(
        result.errors.is_some_and(|errors| !errors.is_empty()) || result.license_string.is_some(),
        "a validation answers with a verdict on what it was given",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn turns_the_monitoring_switches() {
    touch(
        server().monitoring().set_app_monitoring_enabled(AppMonitoringRestEntity { enabled: Some(true) }).send().await,
    );
    touch(
        server().monitoring().set_ipd_monitoring_enabled(IpdMonitoringRestEntity { enabled: Some(true) }).send().await,
    );
    touch(server().monitoring().start().send().await);
    touch(server().monitoring().stop().send().await);

    // The switches answer with nothing, so the read is what carries the assertion: whichever way an instance is
    // configured, it says so with a boolean rather than an absent field.
    let state = server().monitoring().is_app_monitoring_enabled().send().await.expect("the switch reads back");

    assert!(state.enabled.is_some(), "app monitoring reports whether it is on");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn asks_the_cluster_about_itself() {
    // A single node is not a cluster, and Jira says so with a 405 rather than an empty list.
    let nodes = touch(server().cluster().get_all_nodes().send().await);

    assert!(
        nodes.as_ref().is_none_or(|nodes| nodes.iter().all(|node| node.node_id.is_some())),
        "every node in a cluster listing names itself",
    );

    let node_id = nodes
        .and_then(|nodes| nodes.first().and_then(|node| node.node_id.clone()))
        .unwrap_or_else(|| "no-such-node".to_owned());

    touch(server().cluster().change_node_state_to_offline(node_id.clone()).send().await);
    touch(server().cluster().delete_node(node_id).send().await);
    touch(server().cluster().set_ready_to_upgrade().send().await);
    touch(server().cluster().approve_upgrade().send().await);
    touch(server().cluster().acknowledge_errors().send().await);
    touch(server().cluster().cancel_upgrade().send().await);
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn handles_the_email_templates() {
    // "Creates a zip file containing email templates at local home and returns the file", and then the document
    // describes no body — so the operation could be typed as returning nothing and the zip thrown away. The magic
    // number is the assertion: a zip begins `PK\x03\x04`, and nothing else this API answers with does.
    let templates = server().email_templates().download_email_templates().send().await.expect("the templates download");

    assert!(!templates.is_empty(), "a zip of email templates is not empty");
    assert!(templates.starts_with(b"PK\x03\x04"), "the bytes are a zip, not a JSON error");

    // The upload takes a zip; the document describes its body as a JSON object, so there is no zip to send. The gap
    // is the document's, and what is left to prove is that the request reaches Jira in a shape it recognises.
    touch(server().email_templates().upload_email_templates().send().await);
    touch(server().email_templates().apply_email_templates().send().await);
    touch(server().email_templates().revert_email_templates_to_default().send().await);
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_an_avatar_through_the_universal_endpoints() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "avatar owner").await;
    let owner = project.id.to_string();

    let issue_types = server().issue_types().get_issue_all_types().send().await.expect("the instance lists its types");
    let issue_type_id =
        issue_types.first().and_then(|issue_type| issue_type.id.clone()).expect("a Jira instance has issue types");

    let temporary = touch(
        server()
            .avatars()
            .store_temporary_avatar_using_multi_part("project", owner.clone(), [tiny_avatar()])
            .send()
            .await,
    );

    // Called whether or not the upload took: what is under test is the request, and a temporary avatar that is not
    // there is one of the answers this endpoint gives.
    assert!(
        temporary.is_none_or(|cropping| cropping.url.is_some() || cropping.needs_cropping.is_some()),
        "a temporary avatar answers with somewhere to crop it",
    );

    touch(server().avatars().create_avatar_from_temporary("project", owner.clone()).send().await);

    let avatars = server().avatars().get_avatars("project", owner.clone()).send().await.expect("a project has avatars");

    assert!(
        avatars.system.is_some_and(|system| !system.is_empty()),
        "every Jira instance ships system avatars for a project",
    );

    touch(server().avatars().delete_avatar(1, "project", owner).send().await);

    touch(
        server()
            .issue_types()
            .store_temporary_issue_type_avatar_using_multi_part(issue_type_id.clone(), [tiny_avatar()])
            .send()
            .await,
    );
    touch(
        server()
            .issue_types()
            .create_issue_type_avatar_from_temporary(
                issue_type_id,
                AvatarCropping { cropper_width: Some(1), ..AvatarCropping::default() },
            )
            .send()
            .await,
    );
    touch(
        server()
            .projects()
            .create_project_avatar_from_temporary(
                project.key,
                AvatarCropping { cropper_width: Some(1), ..AvatarCropping::default() },
            )
            .send()
            .await,
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn runs_the_upgrade_tasks() {
    touch(server().upgrade().run_upgrades_now().send().await);

    // 404 until an upgrade has actually run, which on a freshly created instance it has not.
    let result = touch(server().upgrade().get_upgrade_result().send().await);

    assert!(
        result.is_none_or(|result| result.outcome.is_some() || result.message.is_some()),
        "an upgrade result says how the upgrade went",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn signs_in_and_out_through_the_session_endpoints() {
    let env = require_server_env();

    // Read unmodelled on purpose: the specification declares `CurrentUser` with no properties at all, so the
    // generated type is an empty struct. The gap is the document's, and the body is what proves it.
    let session = server().session().current_user().send_raw().await.expect("the instance names the current session");

    assert_eq!(
        session["name"].as_str(),
        Some(env.username.as_str()),
        "the session belongs to the account that signed in",
    );

    // A fresh session rather than the one the suite authenticates with, so signing out of it costs nothing — the
    // client sends its credentials on every request and never carries the cookie this hands back.
    touch(
        server()
            .session()
            .login(AuthParams { username: Some(env.username), password: Some(env.password) })
            .send()
            .await,
    );
    touch(server().session().logout().send().await);
    touch(server().websudo().release().send().await);
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn asks_for_an_index_snapshot_and_a_reindex() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "reindex subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to reindex").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    // A snapshot needs somewhere to write it, which a single node with no shared home does not have.
    touch(server().indexing().create_index_snapshot().send().await);

    let requested = touch(server().indexing().reindex_issues().issue_id([key]).send().await);

    assert!(
        requested.is_none_or(|reindex| reindex.success.is_some() || reindex.current_progress.is_some()),
        "a reindex answers with its progress",
    );

    touch(server().indexing().process_requests().send().await);
    touch(server().indexing().get_reindex_request_progress(1).send().await);
    touch(server().indexing().reindex().r#type("BACKGROUND").send().await);

    tracker.cleanup().await;
}

/// Read-only mode is written but never turned on.
///
/// The Data Center API has no endpoint that takes the instance out of read-only mode again, and Rust decides for
/// itself in what order the tests of a binary run — so a suite that enabled it would strand whichever suites happen
/// to run afterwards. The request body, the switch and the read are the same either way; only the value differs.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn writes_the_read_only_mode_switch() {
    touch(
        server()
            .read_only_mode()
            .update_read_only_mode()
            .read_only_mode_update_request(ReadOnlyModeUpdateRequest {
                enabled: Some(false),
                message: Some("written by the Data Center live suite".to_owned()),
                ..ReadOnlyModeUpdateRequest::default()
            })
            .send()
            .await,
    );

    let mode = touch(server().read_only_mode().get_read_only_mode().send().await);

    assert!(mode.is_none_or(|mode| mode.enabled == Some(false)), "the switch reads back the way it was written");
}

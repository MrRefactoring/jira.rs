//! The configuration half of Jira: issue types, link types, screens, and the four kinds of scheme.
//!
//! These are the endpoints a self-hosted administrator reaches for and a Cloud one cannot have, and they are also the
//! least exercised part of the Data Center document — most of them answer with a body it does not describe.
//! Everything created here is registered for deletion the moment it exists, so the instance is left as the other
//! suites expect to find it.

use jira::server::{
    AddField, AssociateProjects, CustomFieldDefinitionJson, DefaultModel, GetWorkflow, IssueLinkTypeJson,
    IssueLinkTypeOrderUpdateRequest, IssueLinkTypeResetOrderRequest, IssueTypeCreate, IssueTypeCreateType,
    IssueTypeMapping, IssueTypeSchemeCreateUpdate, IssueTypeUpdate, MoveField, MoveFieldPosition, PermissionGrant,
    PermissionHolder, PermissionScheme, PrioritySchemeUpdate, ScreenableTab, WorkflowMapping, WorkflowScheme,
};
use serde_json::json;

use crate::harness::{ResourceTracker, project_key, server, test_name};

/// Calls an endpoint for what its response proves, not for whether Jira agrees to do the thing.
///
/// Parts of this surface are administrative in a way a single throwaway node cannot satisfy: a draft of a scheme no
/// project uses, an association to a project that is not there, a priority scheme this edition does not offer. Those
/// operations still have request bodies that must serialise and responses that must match their schemas, and that is
/// what a call proves — Jira answering 400 or 403 proves the request reached it in a shape it recognised.
///
/// A schema mismatch is never swallowed, because it is the one thing these calls exist to catch, and neither is a
/// failure that never reached Jira at all.
fn touch<T>(outcome: jira::Result<T>, what: &str) -> Option<T> {
    match outcome {
        Ok(value) => Some(value),
        Err(error) => {
            assert!(!error.is_schema_mismatch(), "{what} answered a body its schema does not describe: {error}");
            assert!(error.status().is_some(), "{what} failed before reaching Jira: {error}");

            None
        }
    }
}

/// An issue type, from creation to the property hung off it.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_an_issue_type() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .issue_types()
        .create_issue_type(IssueTypeCreate {
            name: Some(test_name("type")),
            description: Some("created by the suite".to_owned()),
            r#type: Some(IssueTypeCreateType::Standard),
        })
        .send()
        .await
        .expect("the instance accepts a new issue type");

    let id = created.id.clone().expect("a created issue type carries an id");

    assert!(id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {id}");
    assert!(created.name.as_ref().is_some_and(|name| name.contains("type")), "creation echoes the name: {created:?}");

    let for_cleanup = id.clone();

    tracker.defer(move || {
        let id = for_cleanup.clone();

        async move { server().issue_types().delete_issue_type(id).send().await }
    });

    server()
        .issue_types()
        .update_issue_type(
            &id,
            IssueTypeUpdate { description: Some("changed by the suite".to_owned()), ..IssueTypeUpdate::default() },
        )
        .send()
        .await
        .expect("the description can be edited");

    let read = server().issue_types().get_issue_type(&id).send().await.expect("the issue type reads back by id");

    assert_eq!(read.description.as_deref(), Some("changed by the suite"), "the edit is observable on the next read");

    let alternatives =
        server().issue_types().get_alternative_issue_types(&id).send().await.expect("the alternatives are listed");

    assert!(
        alternatives.iter().all(|alternative| alternative.id.as_deref() != Some(id.as_str())),
        "a type is never an alternative to itself",
    );

    let property_key = "suite";
    let for_cleanup = id.clone();

    server()
        .issue_types()
        .set_issue_type_property(property_key, &id, [("written".to_owned(), json!(true))].into_iter().collect())
        .send()
        .await
        .expect("a property can be hung off an issue type");

    tracker.defer(move || {
        let id = for_cleanup.clone();

        async move { server().issue_types().delete_issue_type_property(property_key, id).send().await }
    });

    let property = server()
        .issue_types()
        .get_issue_type_property(property_key, &id)
        .send()
        .await
        .expect("the property reads back by key");

    assert_eq!(property.key.as_deref(), Some(property_key));
    assert_eq!(property.value, Some(json!({ "written": true })), "a property comes back as it was written");

    tracker.cleanup().await;
}

/// An issue link type, and the two calls that order the instance's list of them.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_an_issue_link_type() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .issue_link_types()
        .create_issue_link_type(IssueLinkTypeJson {
            name: Some(test_name("link")),
            inward: Some("is blocked by".to_owned()),
            outward: Some("blocks".to_owned()),
            ..IssueLinkTypeJson::default()
        })
        .send()
        .await
        .expect("the instance accepts a new link type");

    let id = created.id.clone().expect("a created link type carries an id");
    let for_cleanup = id.clone();

    tracker.defer(move || {
        let id = for_cleanup.clone();

        async move { server().issue_link_types().delete_issue_link_type(id).send().await }
    });

    server()
        .issue_link_types()
        .update_issue_link_type(
            &id,
            IssueLinkTypeJson {
                name: Some(test_name("link2")),
                inward: Some("depends on".to_owned()),
                outward: Some("is depended on by".to_owned()),
                ..IssueLinkTypeJson::default()
            },
        )
        .send()
        .await
        .expect("both directional phrasings can be edited");

    let read =
        server().issue_link_types().get_issue_link_type(&id).send().await.expect("the link type reads back by id");

    assert_eq!(read.inward.as_deref(), Some("depends on"), "the edit is observable on the next read");
    assert_eq!(read.outward.as_deref(), Some("is depended on by"));

    let moved = touch(
        server()
            .issue_link_types()
            .move_issue_link_type(&id, IssueLinkTypeOrderUpdateRequest { new_position: Some(0) })
            .send()
            .await,
        "moving a link type to the front",
    );

    if let Some(moved) = moved {
        assert_eq!(moved.id.as_deref(), Some(id.as_str()), "moving answers with the type that moved");
    }

    // Read unmodelled on purpose. Atlassian's Data Center specification declares `IssueLinkTypes` with no properties
    // at all, so the generated type is an empty struct and the ordering this call restores would be invisible through
    // it. The gap belongs in the generator's patches; asserting against the body is what proves it is a gap rather
    // than a limit of the client.
    let reset = touch(
        server().issue_link_types().reset_order(IssueLinkTypeResetOrderRequest { direction: None }).send_raw().await,
        "resetting the link type order",
    );

    if let Some(reset) = reset {
        assert!(reset["issueLinkTypes"].is_array(), "the restored order is a list of link types: {reset}");
    }

    tracker.cleanup().await;
}

/// An issue type scheme, and the four calls that attach projects to one.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_an_issue_type_scheme_and_its_project_associations() {
    let mut tracker = ResourceTracker::new();

    let types = server().issue_types().get_issue_all_types().send().await.expect("the instance lists its issue types");
    let standard = types
        .iter()
        .find(|issue_type| issue_type.subtask == Some(false))
        .or_else(|| types.first())
        .expect("a Jira instance always ships with issue types");
    let standard_id = standard.id.clone().expect("an issue type carries an id");

    let created = server()
        .issue_type_schemes()
        .create_issue_type_scheme(IssueTypeSchemeCreateUpdate {
            name: Some(test_name("its")),
            description: Some("created by the suite".to_owned()),
            default_issue_type_id: Some(standard_id.clone()),
            issue_type_ids: Some(vec![standard_id.clone()]),
            ..IssueTypeSchemeCreateUpdate::default()
        })
        .send()
        .await
        .expect("the instance accepts a new issue type scheme");

    let scheme_id = created.id.clone().expect("a created issue type scheme carries an id");
    let for_cleanup = scheme_id.clone();

    tracker.defer(move || {
        let scheme_id = for_cleanup.clone();

        async move { server().issue_type_schemes().delete_issue_type_scheme(scheme_id).send().await }
    });

    server()
        .issue_type_schemes()
        .update_issue_type_scheme(
            &scheme_id,
            IssueTypeSchemeCreateUpdate {
                name: Some(test_name("its2")),
                issue_type_ids: Some(vec![standard_id.clone()]),
                ..IssueTypeSchemeCreateUpdate::default()
            },
        )
        .send()
        .await
        .expect("the scheme can be edited");

    let read = server()
        .issue_type_schemes()
        .get_issue_type_scheme(&scheme_id)
        .send()
        .await
        .expect("the scheme reads back by id");

    assert_eq!(read.id.as_deref(), Some(scheme_id.as_str()), "the scheme read back is the scheme asked for");
    // The instance does not expand the issue types on this read, so what the edit changed is checked through the
    // name it also changed.
    assert!(
        read.name.as_deref().is_some_and(|name| name.contains("its2")),
        "the edit is observable on the next read: {read:?}",
    );
    assert!(
        read.issue_types.as_ref().is_none_or(|issue_types| {
            issue_types.is_empty()
                || issue_types.iter().any(|issue_type| issue_type.id.as_deref() == Some(standard_id.as_str()))
        }),
        "an issue type the read does expand is the one the edit named",
    );

    // An empty association list is the shape the request has to serialise; a project key nothing on this instance
    // answers to is what proves the removal reaches Jira rather than the client.
    let absent = project_key("its");

    touch(
        server()
            .issue_type_schemes()
            .add_project_associations_to_scheme(&scheme_id, AssociateProjects { ids_or_keys: Some(Vec::new()) })
            .send()
            .await,
        "adding project associations",
    );
    touch(
        server()
            .issue_type_schemes()
            .set_project_associations_for_scheme(&scheme_id, AssociateProjects { ids_or_keys: Some(Vec::new()) })
            .send()
            .await,
        "setting project associations",
    );
    touch(
        server().issue_type_schemes().remove_project_association(&absent, &scheme_id).send().await,
        "removing one project association",
    );
    touch(
        server().issue_type_schemes().remove_all_project_associations(&scheme_id).send().await,
        "removing every project association",
    );

    tracker.cleanup().await;
}

/// A permission scheme, one grant inside it, and the attribute hung off the scheme.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_permission_scheme_and_a_grant_in_it() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .permission_schemes()
        .create_permission_scheme()
        .body(PermissionScheme {
            name: Some(test_name("perm")),
            description: Some("created by the suite".to_owned()),
            ..PermissionScheme::default()
        })
        .send()
        .await
        .expect("the instance accepts a new permission scheme");

    let scheme_id = created.id.expect("a created permission scheme carries an id");

    assert!(scheme_id > 0, "a scheme id is a positive number, got {scheme_id}");

    tracker
        .defer(move || async move { server().permission_schemes().delete_permission_scheme(scheme_id).send().await });

    let updated = server()
        .permission_schemes()
        .update_permission_scheme(scheme_id)
        .body(PermissionScheme {
            name: Some(test_name("perm2")),
            description: Some("changed by the suite".to_owned()),
            ..PermissionScheme::default()
        })
        .send()
        .await
        .expect("the scheme can be edited");

    assert_eq!(updated.description.as_deref(), Some("changed by the suite"));

    let grant = server()
        .permission_schemes()
        .create_permission_grant(scheme_id)
        .permission_grant(PermissionGrant {
            permission: Some("BROWSE_PROJECTS".to_owned()),
            holder: Some(PermissionHolder { r#type: Some("anyone".to_owned()), ..PermissionHolder::default() }),
            ..PermissionGrant::default()
        })
        .send()
        .await
        .expect("the scheme accepts a grant");

    let grant_id = grant.id.expect("a created grant carries an id");

    tracker.defer(move || async move {
        server().permission_schemes().delete_permission_scheme_entity(grant_id, scheme_id).send().await
    });

    let grants = server()
        .permission_schemes()
        .get_permission_scheme_grants(scheme_id)
        .send()
        .await
        .expect("the grants in the scheme are listed");

    let listed = grants.permissions.unwrap_or_default();

    assert!(!listed.is_empty(), "a scheme with a grant in it lists at least one");
    assert!(listed.iter().any(|entry| entry.id == Some(grant_id)), "the listing carries the grant just created");

    let attribute_key = "suite";

    server()
        .permission_schemes()
        .set_scheme_attribute(scheme_id, attribute_key)
        .body("true")
        .send()
        .await
        .expect("an attribute can be hung off the scheme");

    let attribute = server()
        .permission_schemes()
        .get_scheme_attribute(scheme_id, attribute_key)
        .send()
        .await
        .expect("the attribute reads back by key");

    // Measured against Data Center 10.3: an attribute outside the set the instance knows is accepted on the way in
    // and answers `false` on the way out. The TypeScript suite asserts the round trip and would fail here too; what
    // is genuinely pinned is that the write is accepted and the read answers with a value at all.
    assert!(attribute.value.is_some(), "the attribute reads back with a value: {attribute:?}");

    tracker.cleanup().await;
}

/// A priority scheme, where the edition offers one.
///
/// Priority schemes are a Data Center feature rather than a universal one, so creation is called for what its request
/// and response prove rather than for the scheme existing afterwards. The priorities it is built from are asserted
/// either way — an instance without those is broken in a way nothing below would explain.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_priority_scheme() {
    let mut tracker = ResourceTracker::new();

    let priorities = server().issue_priorities().get_priorities().send().await.expect("the instance lists priorities");
    let option_ids: Vec<String> = priorities.iter().filter_map(|priority| priority.id.clone()).collect();

    assert!(!option_ids.is_empty(), "a Jira instance always ships with priorities");

    let created = touch(
        server()
            .priority_schemes()
            .create_priority_scheme(PrioritySchemeUpdate {
                name: Some(test_name("prio")),
                description: Some("created by the suite".to_owned()),
                default_option_id: option_ids.first().cloned(),
                option_ids: Some(option_ids.clone()),
                ..PrioritySchemeUpdate::default()
            })
            .send()
            .await,
        "creating a priority scheme",
    );

    let Some(scheme_id) = created.and_then(|scheme| scheme.id) else {
        tracker.cleanup().await;

        return;
    };

    tracker.defer(move || async move { server().priority_schemes().delete_priority_scheme(scheme_id).send().await });

    touch(
        server()
            .priority_schemes()
            .update_priority_scheme(
                scheme_id,
                PrioritySchemeUpdate {
                    name: Some(test_name("prio2")),
                    option_ids: Some(option_ids),
                    ..PrioritySchemeUpdate::default()
                },
            )
            .send()
            .await,
        "editing a priority scheme",
    );

    let read = server()
        .priority_schemes()
        .get_priority_scheme(scheme_id)
        .send()
        .await
        .expect("the priority scheme reads back by id");

    assert_eq!(read.id, Some(scheme_id), "the scheme read back is the scheme asked for");

    tracker.cleanup().await;
}

/// A workflow scheme, its issue type mappings, and the draft half that only exists once a project uses the scheme.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_workflow_scheme_its_draft_and_its_mappings() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .workflow_schemes()
        .create_scheme(WorkflowScheme {
            name: Some(test_name("wf")),
            description: Some("created by the suite".to_owned()),
            ..WorkflowScheme::default()
        })
        .send()
        .await
        .expect("the instance accepts a new workflow scheme");

    let scheme_id = created.id.expect("a created workflow scheme carries an id");

    tracker.defer(move || async move { server().workflow_schemes().delete_scheme(scheme_id).send().await });

    let workflows = server().workflows().get_all_workflows().send().await.expect("the instance lists its workflows");
    let workflow = workflows
        .first()
        .and_then(|workflow| workflow.name.clone())
        .expect("a Jira instance always ships with a workflow");

    server()
        .workflow_schemes()
        .update_workflow_scheme(
            scheme_id,
            WorkflowScheme {
                name: Some(test_name("wf2")),
                description: Some("changed by the suite".to_owned()),
                ..WorkflowScheme::default()
            },
        )
        .send()
        .await
        .expect("the scheme can be edited");

    server()
        .workflow_schemes()
        .update_default(scheme_id, DefaultModel { workflow: Some(workflow.clone()), update_draft_if_needed: None })
        .send()
        .await
        .expect("the scheme takes a default workflow");

    let by_id = server().workflow_schemes().get_by_id(scheme_id).send().await.expect("the scheme reads back by id");

    assert_eq!(by_id.id, Some(scheme_id), "the scheme read back is the scheme asked for");
    assert_eq!(by_id.default_workflow.as_deref(), Some(workflow.as_str()), "the default workflow is the one set");

    let types = server().issue_types().get_issue_all_types().send().await.expect("the instance lists its issue types");
    let issue_type = types.first().and_then(|issue_type| issue_type.id.clone()).expect("an issue type carries an id");

    server()
        .workflow_schemes()
        .set_issue_type(
            &issue_type,
            scheme_id,
            IssueTypeMapping {
                issue_type: Some(issue_type.clone()),
                workflow: Some(workflow.clone()),
                update_draft_if_needed: Some(true),
            },
        )
        .send()
        .await
        .expect("an issue type can be mapped to a workflow");

    let mapping = server()
        .workflow_schemes()
        .get_workflow_scheme_issue_type(&issue_type, scheme_id)
        .send()
        .await
        .expect("the mapping reads back");

    assert_eq!(mapping.workflow.as_deref(), Some(workflow.as_str()), "the mapping names the workflow it was given");

    server()
        .workflow_schemes()
        .update_workflow_mapping(
            scheme_id,
            WorkflowMapping { workflow: Some(workflow.clone()), ..WorkflowMapping::default() },
        )
        .workflow_name(&workflow)
        .send()
        .await
        .expect("the mapping can be edited by workflow name");

    // Naming a workflow narrows the answer to that one mapping; leaving it out lists them all. The endpoint returns
    // both shapes and the generated union says so.
    let for_workflow = server()
        .workflow_schemes()
        .get_workflow(scheme_id)
        .workflow_name(&workflow)
        .send()
        .await
        .expect("the mapping for one workflow reads back");

    let named = match for_workflow {
        GetWorkflow::WorkflowMapping(mapping) => mapping.workflow,
        GetWorkflow::Variant1(mappings) => mappings.into_iter().next().and_then(|mapping| mapping.workflow),
        other => panic!("naming a workflow answers with a mapping, got {other:?}"),
    };

    assert_eq!(named.as_deref(), Some(workflow.as_str()), "the narrowed answer is about the workflow asked for");

    let all = server().workflow_schemes().get_workflow(scheme_id).send().await.expect("every mapping reads back");

    assert!(matches!(&all, GetWorkflow::Variant1(_)), "leaving the workflow out lists them all, got {all:?}");

    touch(
        server().workflow_schemes().delete_workflow_scheme_issue_type(&issue_type, scheme_id).send().await,
        "unmapping an issue type",
    );
    touch(
        server().workflow_schemes().delete_workflow_mapping(scheme_id).workflow_name(&workflow).send().await,
        "deleting a workflow mapping",
    );
    touch(server().workflow_schemes().delete_default(scheme_id).send().await, "clearing the default workflow");

    // A draft only exists once the scheme is in use by a project; every one of these is a legitimate refusal on a
    // scheme that is not.
    touch(server().workflow_schemes().create_draft_for_parent(scheme_id).send().await, "creating a draft");
    touch(server().workflow_schemes().get_draft_by_id(scheme_id).send().await, "reading a draft");
    touch(
        server()
            .workflow_schemes()
            .update_draft(scheme_id, WorkflowScheme { name: Some(test_name("wf3")), ..WorkflowScheme::default() })
            .send()
            .await,
        "editing a draft",
    );
    touch(
        server()
            .workflow_schemes()
            .update_draft_default(
                scheme_id,
                DefaultModel { workflow: Some(workflow.clone()), update_draft_if_needed: None },
            )
            .send()
            .await,
        "setting a draft's default workflow",
    );
    touch(server().workflow_schemes().get_draft_default(scheme_id).send().await, "reading a draft's default workflow");
    touch(
        server()
            .workflow_schemes()
            .set_draft_issue_type(
                &issue_type,
                scheme_id,
                IssueTypeMapping {
                    issue_type: Some(issue_type.clone()),
                    workflow: Some(workflow.clone()),
                    update_draft_if_needed: None,
                },
            )
            .send()
            .await,
        "mapping an issue type in a draft",
    );
    touch(
        server().workflow_schemes().get_draft_issue_type(&issue_type, scheme_id).send().await,
        "reading a draft's issue type mapping",
    );
    touch(
        server()
            .workflow_schemes()
            .update_draft_workflow_mapping(
                scheme_id,
                WorkflowMapping { workflow: Some(workflow.clone()), ..WorkflowMapping::default() },
            )
            .workflow_name(&workflow)
            .send()
            .await,
        "editing a draft's workflow mapping",
    );
    touch(server().workflow_schemes().get_draft_workflow(scheme_id).send().await, "reading a draft's mappings");
    touch(
        server().workflow_schemes().delete_draft_issue_type(&issue_type, scheme_id).send().await,
        "unmapping an issue type in a draft",
    );
    touch(
        server().workflow_schemes().delete_draft_workflow_mapping(scheme_id).workflow_name(&workflow).send().await,
        "deleting a draft's workflow mapping",
    );
    touch(
        server().workflow_schemes().delete_draft_default(scheme_id).send().await,
        "clearing a draft's default workflow",
    );
    touch(server().workflow_schemes().delete_draft_by_id(scheme_id).send().await, "deleting a draft");

    tracker.cleanup().await;
}

/// A tab on a screen, the fields moved around on it, and the custom field put on the default screen.
///
/// The custom field is made here rather than borrowed: adding a field to the default screen changes the create dialog
/// for every user of every project, so what is added has to be something this test can take away again.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn adds_a_tab_to_a_screen_and_takes_it_away() {
    let mut tracker = ResourceTracker::new();

    let screens = server().screens().get_all_screens().send().await.expect("the instance lists its screens");
    let screen_id = screens.first().and_then(|screen| screen.id).expect("a Jira instance always ships with screens");

    let tab = server()
        .screens()
        .add_tab(screen_id)
        .screenable_tab(ScreenableTab { name: Some(test_name("tab")), id: None })
        .send()
        .await
        .expect("the screen accepts a new tab");

    let tab_id = tab.id.expect("a created tab carries an id");

    tracker.defer(move || async move { server().screens().delete_tab(tab_id, screen_id).send().await });

    let renamed_to = test_name("tab2");
    let renamed = server()
        .screens()
        .rename_tab(tab_id, screen_id)
        .screenable_tab(ScreenableTab { name: Some(renamed_to.clone()), id: None })
        .send()
        .await
        .expect("the tab can be renamed");

    assert_eq!(renamed.name.as_deref(), Some(renamed_to.as_str()), "renaming answers with the new name");

    touch(server().screens().move_tab(tab_id, screen_id, 0).send().await, "moving the tab to the front");

    let available =
        server().screens().get_fields_to_add(screen_id).send().await.expect("the fields not yet on the screen list");

    if let Some(field_id) = available.first().and_then(|field| field.id.clone()) {
        touch(
            server()
                .screens()
                .add_field(tab_id, screen_id)
                .add_field(AddField { field_id: Some(field_id.clone()) })
                .send()
                .await,
            "adding a field to the tab",
        );
        touch(
            server()
                .screens()
                .move_field(tab_id, screen_id, &field_id)
                .move_field(MoveField { position: Some(MoveFieldPosition::First), after: None })
                .send()
                .await,
            "moving a field to the front of the tab",
        );
        touch(
            server().screens().update_show_when_empty_indicator(tab_id, screen_id, true, &field_id).send().await,
            "showing the field when it is empty",
        );
        touch(
            server().screens().remove_field(tab_id, screen_id, &field_id).send().await,
            "taking the field off the tab",
        );
    }

    let custom_field = server()
        .issue_fields()
        .create_custom_field()
        .custom_field_definition_json(CustomFieldDefinitionJson {
            name: Some(test_name("screen field")),
            description: Some("created by the suite".to_owned()),
            r#type: Some("com.atlassian.jira.plugin.system.customfieldtypes:textfield".to_owned()),
            searcher_key: Some("com.atlassian.jira.plugin.system.customfieldtypes:textsearcher".to_owned()),
            ..CustomFieldDefinitionJson::default()
        })
        .send()
        .await
        .expect("the instance accepts a new custom field");

    let custom_field_id = custom_field.id.clone().expect("a created custom field carries an id");
    let for_cleanup = custom_field_id.clone();

    tracker.defer(move || {
        let ids = for_cleanup.clone();

        async move { server().issue_fields().bulk_delete_custom_fields(ids).send().await.map(|_| ()) }
    });

    touch(
        server().screens().add_field_to_default_screen(&custom_field_id).send().await,
        "adding the custom field to the default screen",
    );

    tracker.cleanup().await;
}

/// The bulk delete, which takes its ids as one comma-separated parameter rather than as a body.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn deletes_a_custom_field_in_bulk() {
    let mut tracker = ResourceTracker::new();

    let created = server()
        .issue_fields()
        .create_custom_field()
        .custom_field_definition_json(CustomFieldDefinitionJson {
            name: Some(test_name("bulk field")),
            description: Some("created by the suite".to_owned()),
            r#type: Some("com.atlassian.jira.plugin.system.customfieldtypes:textfield".to_owned()),
            searcher_key: Some("com.atlassian.jira.plugin.system.customfieldtypes:textsearcher".to_owned()),
            ..CustomFieldDefinitionJson::default()
        })
        .send()
        .await
        .expect("the instance accepts a new custom field");

    let id = created.id.clone().expect("a created custom field carries an id");

    assert!(id.starts_with("customfield_"), "a custom field is addressed by a prefixed id: {id}");

    let for_cleanup = id.clone();

    tracker.defer(move || {
        let ids = for_cleanup.clone();

        async move { server().issue_fields().bulk_delete_custom_fields(ids).send().await.map(|_| ()) }
    });

    let deleted =
        touch(server().issue_fields().bulk_delete_custom_fields(&id).send().await, "deleting a custom field in bulk");

    if let Some(deleted) = deleted {
        assert!(
            deleted.deleted_custom_fields.iter().flatten().any(|removed| removed == &id),
            "the bulk delete names what it removed: {deleted:?}",
        );
    }

    tracker.cleanup().await;
}

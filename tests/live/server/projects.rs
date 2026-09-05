//! Projects and everything hung off one: categories, roles, components, versions and avatars.
//!
//! Every test creates a project of its own, because half of what is under test here changes the project itself — its
//! type, its permission scheme, its actors — and because a bare Data Center instance has no project to borrow. What
//! the TypeScript suite did once in a `beforeAll` is done per test and registered for removal the moment it exists.

use jira::server::{
    ActorInput, ActorsMap, Avatar, AvatarCropping, Component, CreateUpdateRoleRequest, DeleteAndReplaceVersion, Id,
    ProjectCategory, ProjectRoleActorsUpdate, ProjectUpdate, RemoteEntityLinkJson, Version, VersionMove,
    VersionMovePosition,
};
use serde_json::json;

use super::fixtures::{admin_username, business_project, property_body, property_value, tiny_avatar, touch};
use crate::harness::{ResourceTracker, server, test_name};

/// The global id the remote version links in this suite are written under.
const VERSION_LINK_GLOBAL_ID: &str = "jrs-version-link";

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_the_project_it_created_and_what_belongs_to_it() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "projects suite").await;

    let read = server().projects().get_project(&project.key).send().await.expect("the project reads back by key");

    assert_eq!(read.key.as_deref(), Some(project.key.as_str()), "the project read back is the one created");
    assert_eq!(read.id.as_deref(), Some(project.id.to_string().as_str()), "and it carries the id creation handed back");

    let statuses = server().projects().get_all_statuses(&project.key).send().await.expect("the statuses read");

    assert!(!statuses.is_empty(), "a project has issue types");
    assert!(
        statuses.iter().all(|issue_type| issue_type.statuses.as_ref().is_some_and(|statuses| !statuses.is_empty())),
        "and every issue type of a project carries the statuses its workflow allows",
    );

    let roles = server().projects().get_project_roles(&project.key).send().await.expect("the roles read");

    assert!(!roles.is_empty(), "a project has roles");
    assert!(roles.values().all(|url| url.contains("/role/")), "each role is named by the url that addresses it");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn updates_the_project_and_its_type() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "project to rename").await;
    let name = test_name("renamed project");

    server()
        .projects()
        .update_project(
            &project.key,
            ProjectUpdate {
                name: Some(name.clone()),
                description: Some("changed by the suite".to_owned()),
                ..ProjectUpdate::default()
            },
        )
        .send()
        .await
        .expect("a project can be renamed");

    let read = server().projects().get_project(&project.key).send().await.expect("the renamed project reads back");

    assert_eq!(read.name.as_deref(), Some(name.as_str()), "the rename is observable on the next read");
    assert_eq!(read.description.as_deref(), Some("changed by the suite"), "and so is the description");

    // Changing a business project into a software one needs the Software application installed and licensed.
    touch(server().projects().update_project_type(&project.key, "software").send().await);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn assigns_the_schemes_a_project_can_hold() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "scheme holder").await;

    let schemes = server().permission_schemes().get_permission_schemes().send().await.expect("the schemes read");
    let scheme_id = schemes
        .permission_schemes
        .as_ref()
        .and_then(|schemes| schemes.first())
        .and_then(|scheme| scheme.id)
        .expect("a Jira instance ships a default permission scheme");

    server()
        .projects()
        .assign_permission_scheme(&project.key, Id { id: Some(scheme_id) })
        .send()
        .await
        .expect("a permission scheme can be assigned to a project");

    let assigned = server()
        .projects()
        .get_assigned_permission_scheme(&project.key)
        .send()
        .await
        .expect("the assigned scheme reads back");

    assert_eq!(assigned.id, Some(scheme_id), "the scheme the project holds is the one that was assigned");

    let priorities =
        server().priority_schemes().get_priority_schemes().send().await.expect("the priority schemes read");

    assert!(
        priorities.schemes.iter().flatten().all(|scheme| scheme.id.is_some() && scheme.name.is_some()),
        "every priority scheme in a listing is addressable and named",
    );

    let priority_id = priorities.schemes.as_ref().and_then(|schemes| schemes.first()).and_then(|scheme| scheme.id);

    // Priority schemes are a Data Center feature a timebomb licence does not always carry.
    if let Some(id) = priority_id {
        touch(server().projects().assign_priority_scheme(&project.key, Id { id: Some(id) }).send().await);
        touch(server().projects().unassign_priority_scheme(id, &project.key).send().await);
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn stores_a_property_on_the_project() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "property holder").await;

    server()
        .projects()
        .set_project_property("suite", &project.key, property_body())
        .send()
        .await
        .expect("a project takes a property of the caller's own");

    let property =
        server().projects().get_project_property("suite", &project.key).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    server()
        .projects()
        .delete_project_property("suite", &project.key)
        .send()
        .await
        .expect("the property can be removed");

    let error = server()
        .projects()
        .get_project_property("suite", &project.key)
        .send()
        .await
        .expect_err("a removed property cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn puts_actors_in_a_role_and_takes_them_out() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "role holder").await;
    let username = admin_username();

    let roles = server().projects().get_project_roles(&project.key).send().await.expect("the roles read");
    let url = roles.values().next().expect("a project has at least one role");
    let role_id: i64 =
        url.rsplit('/').next().and_then(|tail| tail.parse().ok()).expect("a role url ends in the role's id");

    touch(
        server()
            .projects()
            .add_actor_users(
                &project.key,
                role_id,
                ActorsMap { user: Some(vec![username.clone()]), ..ActorsMap::default() },
            )
            .send()
            .await,
    );
    touch(
        server()
            .projects()
            .set_actors(
                &project.key,
                role_id,
                ProjectRoleActorsUpdate {
                    categorised_actors: Some(
                        [("atlassian-user-role-actor".to_owned(), json!([username]))].into_iter().collect(),
                    ),
                    ..ProjectRoleActorsUpdate::default()
                },
            )
            .send()
            .await,
    );

    let role = server().projects().get_project_role(&project.key, role_id).send().await.expect("the role reads back");

    assert_eq!(role.id, Some(role_id), "the role read back is the one addressed");

    touch(server().projects().delete_actor(&project.key, role_id).user(username).send().await);

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_category() {
    let mut tracker = ResourceTracker::new();

    let category = server()
        .project_categories()
        .create_project_category(ProjectCategory {
            name: Some(test_name("category")),
            description: Some("created by the suite".to_owned()),
            ..ProjectCategory::default()
        })
        .send()
        .await
        .expect("a project category can be created");

    let id: i64 =
        category.id.as_deref().and_then(|id| id.parse().ok()).expect("a created category carries a numeric id");

    tracker.defer(move || async move { server().project_categories().remove_project_category(id).send().await });

    let renamed = test_name("renamed category");

    server()
        .project_categories()
        .update_project_category(id, ProjectCategory { name: Some(renamed.clone()), ..ProjectCategory::default() })
        .send()
        .await
        .expect("a category can be renamed");

    let read = server()
        .project_categories()
        .get_project_category_by_id(id)
        .send()
        .await
        .expect("the category reads back by id");

    assert_eq!(read.id.as_deref(), category.id.as_deref(), "the category read back is the one created");
    assert_eq!(read.name.as_deref(), Some(renamed.as_str()), "the rename is observable on the next read");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_component() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "component holder").await;

    let component = server()
        .project_components()
        .create_component()
        .component(Component {
            name: Some(test_name("component")),
            project: Some(project.key.clone()),
            description: Some("created by the suite".to_owned()),
            ..Component::default()
        })
        .send()
        .await
        .expect("a component can be created");

    let id = component.id.clone().expect("a created component carries an id");
    let doomed = id.clone();

    tracker.defer(move || {
        let id = doomed.clone();

        async move { server().project_components().delete_component(id).send().await }
    });

    server()
        .project_components()
        .update_component(&id)
        .body(Component { description: Some("changed by the suite".to_owned()), ..Component::default() })
        .send()
        .await
        .expect("a component can be changed");

    let read = server().project_components().get_component(&id).send().await.expect("the component reads back");

    assert_eq!(read.description.as_deref(), Some("changed by the suite"), "the change is observable on the next read");

    let related =
        server().project_components().get_component_related_issues(&id).send().await.expect("the issue count reads");

    assert_eq!(related.issue_count, Some(0), "a component nothing was filed against holds no issues");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_version_moves_it_and_merges_it_away() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "version holder").await;

    let version = create_version(&mut tracker, &project.key, "v1").await;
    let id = version.id.clone().expect("a created version carries an id");

    server()
        .project_versions()
        .update_version(&id, Version { description: Some("changed by the suite".to_owned()), ..Version::default() })
        .send()
        .await
        .expect("a version can be changed");

    let read = server().project_versions().get_version(&id).send().await.expect("the version reads back");

    assert_eq!(read.description.as_deref(), Some("changed by the suite"), "the change is observable on the next read");

    server()
        .project_versions()
        .move_version(&id, VersionMove { position: Some(VersionMovePosition::First), ..VersionMove::default() })
        .send()
        .await
        .expect("a version can be moved to the front of the project's sequence");

    let unresolved =
        server().project_versions().get_version_unresolved_issues(&id).send().await.expect("the count reads");

    assert_eq!(unresolved.issues_unresolved_count, Some(0), "a version nothing is filed against has nothing open");

    let related = server().project_versions().get_version_related_issues(&id).send().await.expect("the counts read");

    assert!(related.self_.is_some(), "a related-issues count names the version it counts for");

    let other = create_version(&mut tracker, &project.key, "v2").await;
    let other_id = other.id.clone().expect("a created version carries an id");

    server().project_versions().merge(&other_id, &id).send().await.expect("one version can be merged into another");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_remote_links_on_a_version() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "version link holder").await;

    let version = create_version(&mut tracker, &project.key, "v3").await;
    let id = version.id.clone().expect("a created version carries an id");

    server()
        .project_versions()
        .create_or_update_remote_version_link_by_global_id(
            &id,
            VERSION_LINK_GLOBAL_ID,
            RemoteEntityLinkJson {
                link: Some(json!({ "url": "https://github.com/MrRefactoring/jirars", "name": "jirars" })),
                ..RemoteEntityLinkJson::default()
            },
        )
        .send()
        .await
        .expect("a remote version link can be written under a global id of the caller's choosing");

    // Without a global id Jira generates one, and the document describes the payload as a wrapper around the link
    // rather than as the link itself — so what this proves is that the request reaches Jira in a shape it knows.
    touch(
        server()
            .project_versions()
            .create_or_update_remote_version_link(
                &id,
                RemoteEntityLinkJson {
                    link: Some(json!({ "url": "https://example.com/one", "name": "one" })),
                    ..RemoteEntityLinkJson::default()
                },
            )
            .send()
            .await,
    );

    let links = server()
        .project_versions()
        .get_remote_version_links_by_version_id(&id)
        .send()
        .await
        .expect("the remote links of a version read back");

    assert!(
        links.links.as_ref().is_some_and(|links| !links.is_empty()),
        "the link just written is in the listing: {links:?}",
    );

    let read = server()
        .project_versions()
        .get_remote_version_link(&id, VERSION_LINK_GLOBAL_ID)
        .send()
        .await
        .expect("a remote link reads back by its global id");

    assert!(read.link.is_some(), "a remote version link carries the payload it was written with");

    server()
        .project_versions()
        .delete_remote_version_link(&id, VERSION_LINK_GLOBAL_ID)
        .send()
        .await
        .expect("a remote link can be removed by its global id");

    server()
        .project_versions()
        .delete_remote_version_links_by_version_id(&id)
        .send()
        .await
        .expect("and the rest can be removed in one go");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn archives_and_restores() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "archive subject").await;

    // Archiving a project needs Data Center licensing a timebomb does not always carry.
    touch(server().projects().archive_project(&project.key).send().await);
    touch(server().projects().restore_project(&project.key).send().await);

    let read = server().projects().get_project(&project.key).send().await.expect("the project is readable afterwards");

    assert_eq!(read.key.as_deref(), Some(project.key.as_str()), "the project survived the round trip");
    assert_ne!(read.archived, Some(true), "and it is not left archived");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn uploads_an_avatar_and_puts_it_back() {
    let mut tracker = ResourceTracker::new();
    let project = business_project(&mut tracker, "avatar holder").await;

    let temporary = touch(
        server().projects().store_temporary_project_avatar_using_multi_part(&project.key, [tiny_avatar()]).send().await,
    );

    if temporary.is_some() {
        touch(
            server()
                .projects()
                .create_project_avatar_from_temporary(
                    &project.key,
                    AvatarCropping { cropper_width: Some(1), ..AvatarCropping::default() },
                )
                .send()
                .await,
        );
    }

    let avatars =
        server().projects().get_all_project_avatars(&project.key).send().await.expect("the avatars of a project read");
    let system = avatars.system.as_ref().expect("a project has system avatars");

    assert!(!system.is_empty(), "every Jira instance ships system avatars");

    let id = system.first().and_then(|avatar| avatar.id.clone()).expect("a system avatar is addressed by an id");

    touch(
        server()
            .projects()
            .update_project_avatar(&project.key, Avatar { id: Some(id.clone()), ..Avatar::default() })
            .send()
            .await,
    );

    if let Ok(id) = id.parse::<i64>() {
        // A system avatar cannot be deleted, which is the refusal this proves is typed.
        touch(server().projects().delete_project_avatar(&project.key, id).send().await);
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_a_project_role_of_its_own() {
    let mut tracker = ResourceTracker::new();
    let username = admin_username();

    let role = server()
        .project_roles()
        .create_project_role(CreateUpdateRoleRequest {
            name: Some(test_name("role")),
            description: Some("created by the suite".to_owned()),
        })
        .send()
        .await
        .expect("a project role can be created");

    let id = role.id.expect("a created role carries an id");

    tracker.defer(move || async move { server().project_roles().delete_project_role(id).send().await });

    server()
        .project_roles()
        .partial_update_project_role(id)
        .create_update_role_request(CreateUpdateRoleRequest {
            description: Some("changed by the suite".to_owned()),
            ..CreateUpdateRoleRequest::default()
        })
        .send()
        .await
        .expect("a role can be changed in part");

    let renamed = test_name("renamed role");

    let replaced = server()
        .project_roles()
        .fully_update_project_role(id)
        .create_update_role_request(CreateUpdateRoleRequest {
            name: Some(renamed.clone()),
            description: Some("replaced by the suite".to_owned()),
        })
        .send()
        .await
        .expect("a role can be replaced wholesale");

    assert_eq!(replaced.name.as_deref(), Some(renamed.as_str()), "the replacement carries the name it was given");
    assert_eq!(replaced.description.as_deref(), Some("replaced by the suite"), "and the description with it");

    touch(
        server()
            .project_roles()
            .add_project_role_actors_to_role(id)
            .actor_input(ActorInput { user: Some(vec![username.clone()]), ..ActorInput::default() })
            .send()
            .await,
    );
    touch(server().project_roles().delete_project_role_actors_from_role(id).user(username).send().await);

    tracker.cleanup().await;
}

/// Creates a version in the project and registers the removal of it.
///
/// Data Center has no plain delete for a version: the only removal is `removeAndSwap`, which is a POST with a body
/// describing what to do with whatever pointed at the version — nothing, here, since nothing does.
async fn create_version(tracker: &mut ResourceTracker, project_key: &str, label: &str) -> Version {
    let version = server()
        .project_versions()
        .create_version(Version {
            name: Some(test_name(label)),
            project: Some(project_key.to_owned()),
            ..Version::default()
        })
        .send()
        .await
        .expect("a version can be created");

    let id = version.id.clone().expect("a created version carries an id");

    tracker.defer(move || {
        let id = id.clone();

        async move {
            server().project_versions().delete_version_and_swap(id, DeleteAndReplaceVersion::default()).send().await
        }
    });

    version
}

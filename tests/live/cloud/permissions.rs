use jira::cloud::{
    BulkPermissionsRequest, BulkProjectPermissions, GetMyPermissionsRequestPermissions, PermissionsKeys,
};
use serde_json::Value;

use crate::harness::{TEST_PROJECT_KEY, cloud};

/// Every permission the API returns carries a key and a type; `havePermission` is what callers branch on.
fn assert_well_formed(permission: &Value) {
    let key = permission.get("key").and_then(Value::as_str).expect("a permission carries a key");

    assert!(!key.is_empty(), "a permission key is not blank");

    let scope = permission.get("type").and_then(Value::as_str).expect("a permission carries a type");

    assert!(matches!(scope, "GLOBAL" | "PROJECT"), "a permission is global or project scoped, got {scope}");
}

/// Whether the caller holds the permission, as the entry reports it.
fn have_permission(permission: &Value) -> Option<bool> {
    permission.get("havePermission").and_then(Value::as_bool)
}

/// What the token can actually do, pinned in one place.
///
/// Several other suites gate their write paths on exactly these answers, so a silent loss of a permission would
/// otherwise surface as a confusing 403 three files away.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_for_a_named_permission_in_the_scope_of_the_test_project() {
    let result = cloud()
        .permissions()
        .get_my_permissions()
        .project_key(TEST_PROJECT_KEY)
        .permissions(GetMyPermissionsRequestPermissions::Many(vec![
            "BROWSE_PROJECTS".to_owned(),
            "CREATE_ISSUES".to_owned(),
        ]))
        .send()
        .await
        .expect("the site reports what the token may do in the test project");

    let permissions = result.permissions.expect("the answer carries the permissions asked for");
    let mut keys: Vec<_> = permissions.keys().cloned().collect();

    keys.sort();

    assert_eq!(keys, ["BROWSE_PROJECTS", "CREATE_ISSUES"]);

    for permission in permissions.values() {
        assert_well_formed(permission);
    }

    assert_eq!(have_permission(&permissions["BROWSE_PROJECTS"]), Some(true));
    assert_eq!(have_permission(&permissions["CREATE_ISSUES"]), Some(true));
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn narrows_to_exactly_the_permissions_asked_for_not_the_whole_catalogue() {
    let result = cloud()
        .permissions()
        .get_my_permissions()
        .project_key(TEST_PROJECT_KEY)
        .permissions(GetMyPermissionsRequestPermissions::Many(vec!["BROWSE_PROJECTS".to_owned()]))
        .send()
        .await
        .expect("one permission is a valid request");

    let keys: Vec<_> = result.permissions.unwrap_or_default().into_keys().collect();

    assert_eq!(keys, ["BROWSE_PROJECTS"]);
}

/// A project permission means something different in and out of project scope: globally it is "in any project", in a
/// project it is "in this one". Callers that conflate the two ship a UI that offers actions the save will reject.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_project_permissions_differently_in_and_out_of_project_scope() {
    let scoped = cloud()
        .permissions()
        .get_my_permissions()
        .project_key(TEST_PROJECT_KEY)
        .permissions(GetMyPermissionsRequestPermissions::Many(vec!["CREATE_ISSUES".to_owned()]))
        .send()
        .await
        .expect("the scoped question is answered");

    let global = cloud()
        .permissions()
        .get_my_permissions()
        .permissions(GetMyPermissionsRequestPermissions::Many(vec!["CREATE_ISSUES".to_owned()]))
        .send()
        .await
        .expect("the same question is answered without a project");

    let scoped = scoped.permissions.expect("the scoped answer carries permissions");
    let global = global.permissions.expect("the global answer carries permissions");

    assert_eq!(
        have_permission(&scoped["CREATE_ISSUES"]),
        Some(true),
        "the suite creates issues in the test project, so it may",
    );
    assert!(have_permission(&global["CREATE_ISSUES"]).is_some(), "the global answer still decides one way or another");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_unknown_permission_key_rather_than_silently_ignoring_it() {
    let error = cloud()
        .permissions()
        .get_my_permissions()
        .permissions(GetMyPermissionsRequestPermissions::Many(vec!["NO_SUCH_PERMISSION".to_owned()]))
        .send()
        .await
        .expect_err("a permission that does not exist cannot be answered for");

    assert_eq!(error.status(), Some(400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_site_permission_catalogue_every_entry_well_formed() {
    let all = cloud().permissions().get_all_permissions().send().await.expect("the site lists its permissions");
    let permissions = all.permissions.expect("the catalogue carries entries");

    assert!(permissions.len() > 20, "the catalogue is the whole set, got {} entries", permissions.len());

    for permission in permissions.values() {
        assert_well_formed(permission);
    }

    for expected in ["BROWSE_PROJECTS", "CREATE_ISSUES", "DELETE_ISSUES", "ADMINISTER"] {
        assert!(permissions.contains_key(expected), "the catalogue names {expected}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_global_and_project_permissions_in_one_round_trip() {
    let result = cloud()
        .permissions()
        .get_bulk_permissions(BulkPermissionsRequest {
            global_permissions: Some(vec!["ADMINISTER".to_owned()]),
            project_permissions: Some(vec![BulkProjectPermissions {
                permissions: vec!["BROWSE_PROJECTS".to_owned()],
                projects: Some(Vec::new()),
                issues: None,
            }]),
            ..BulkPermissionsRequest::default()
        })
        .send()
        .await
        .expect("global and project permissions resolve together");

    assert!(
        result.global_permissions.iter().all(|permission| permission == "ADMINISTER"),
        "only what was asked for comes back: {:?}",
        result.global_permissions,
    );
    assert!(
        result.project_permissions.iter().all(|grant| grant.permission == "BROWSE_PROJECTS"),
        "only what was asked for comes back: {:?}",
        result.project_permissions,
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_an_empty_request_with_empty_results_rather_than_an_error() {
    let result = cloud()
        .permissions()
        .get_bulk_permissions(BulkPermissionsRequest::default())
        .send()
        .await
        .expect("an empty request is a valid request");

    assert!(result.global_permissions.is_empty(), "{:?}", result.global_permissions);
    assert!(result.project_permissions.is_empty(), "{:?}", result.project_permissions);
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn includes_the_test_project_among_those_the_token_can_create_issues_in() {
    let result = cloud()
        .permissions()
        .get_permitted_projects(PermissionsKeys { permissions: vec!["CREATE_ISSUES".to_owned()] })
        .send()
        .await
        .expect("the site lists where the token may create issues");

    let keys: Vec<_> = result.projects.unwrap_or_default().into_iter().filter_map(|project| project.key).collect();

    assert!(keys.iter().any(|key| key == TEST_PROJECT_KEY), "the project the suites run in is among them: {keys:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_unknown_permission_key_when_listing_projects() {
    let error = cloud()
        .permissions()
        .get_permitted_projects(PermissionsKeys { permissions: vec!["NO_SUCH_PERMISSION".to_owned()] })
        .send()
        .await
        .expect_err("a permission that does not exist grants nothing anywhere");

    assert_eq!(error.status(), Some(400), "{error}");
}

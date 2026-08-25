use jira::service_desk_server::{RequestTypeCreate, RequestTypePermissionRequest, RequestTypeUpdate};

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::{jsm_platform, service_desk_server};

async fn any_issue_type_id() -> String {
    let types = jsm_platform()
        .issue_types()
        .get_issue_all_types()
        .send()
        .await
        .expect("the instance lists the issue types it declares");

    types
        .into_iter()
        .find_map(|issue_type| issue_type.id)
        .expect("an instance with a service desk project declares issue types")
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_request_types_and_their_groups() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();

    let types = service_desk_server()
        .request_types()
        .get_request_types(desk.clone())
        .send()
        .await
        .expect("a service desk lists its request types");

    assert!(!types.values.is_empty(), "the service desk template brings request types with it");

    let groups = service_desk_server()
        .request_types()
        .get_request_type_groups(desk)
        .send()
        .await
        .expect("a service desk lists the groups its request types are filed under");

    assert!(groups.values.iter().all(|group| group.id.is_some()), "every group is addressable by an id");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_reads_updates_and_deletes_a_request_type() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();
    let issue_type_id = any_issue_type_id().await;

    let created = service_desk_server()
        .request_types()
        .create_request_type(desk.clone())
        .request_type_create(RequestTypeCreate {
            issue_type_id: Some(issue_type_id),
            name: Some(asset_name("request type")),
            description: Some("Created and removed by one test.".to_owned()),
            help_text: Some("Nothing to see here.".to_owned()),
        })
        .send()
        .await
        .expect("a service desk accepts a request type");

    let id = created.id.clone().expect("a created request type carries an id");

    let loaded = service_desk_server()
        .request_types()
        .get_request_type_by_id(desk.clone(), id.clone())
        .send()
        .await
        .expect("a request type reads back by id");

    assert_eq!(loaded.id, created.id, "the request type read back is the one asked for");

    let fields = service_desk_server()
        .request_types()
        .get_request_type_fields(desk.clone(), id.clone())
        .send()
        .await
        .expect("a request type lists the fields its form asks for");

    let _ = fields;

    let updated = service_desk_server()
        .request_types()
        .update_request_type(desk.clone())
        .request_type_update(RequestTypeUpdate {
            request_type_id: id.parse().ok(),
            name: Some(asset_name("renamed request type")),
            description: Some("Updated by one test.".to_owned()),
            help_text: None,
        })
        .send()
        .await
        .expect("a request type can be rewritten");

    assert!(
        updated.name.as_deref().is_some_and(|name| name.contains("renamed request type")),
        "the name the body carried is the name that stuck",
    );

    service_desk_server()
        .request_types()
        .delete_request_type(desk, id)
        .send()
        .await
        .expect("a request type can be removed");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reads_and_writes_the_permissions_of_a_request_type() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();

    let types = service_desk_server()
        .request_types()
        .get_request_types(desk.clone())
        .send()
        .await
        .expect("a service desk lists its request types");

    let id = types.values.first().and_then(|kind| kind.id.clone()).expect("the template brings request types with it");

    let permissions = service_desk_server()
        .request_type_permissions()
        .get_permissions_by_request_type_id(desk.clone(), id.clone())
        .send()
        .await
        .expect("a request type reports who may raise it");

    assert_eq!(permissions.id.as_deref(), Some(id.as_str()), "the permissions read back belong to the type asked for");

    service_desk_server()
        .request_type_permissions()
        .upsert_request_type_permission(desk, id, RequestTypePermissionRequest { allowlist: Some(Vec::new()) })
        .send()
        .await
        .expect("the permissions of a request type can be written back");
}

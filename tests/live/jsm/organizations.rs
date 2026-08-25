use jira::service_desk_server::{OrganizationCreate, OrganizationServiceDeskUpdate, UsersOrganizationUpdate};

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::service_desk_server;

async fn create_organization(label: &str) -> i64 {
    let created = service_desk_server()
        .organizations()
        .create_organization()
        .organization_create(OrganizationCreate { name: Some(asset_name(label)) })
        .send()
        .await
        .expect("the instance accepts an organization");

    created.id.expect("a created organization carries an id").parse().expect("an organization id is a number")
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_lists_loads_and_deletes_an_organization() {
    if !service_desk_licensed().await {
        return;
    }

    let id = create_organization("organization").await;

    let listed = service_desk_server()
        .organizations()
        .get_organizations()
        .send()
        .await
        .expect("the instance lists its organizations");

    assert!(
        listed.values.iter().any(|organization| organization.id.as_deref() == Some(id.to_string().as_str())),
        "the organization just made is among them",
    );

    let loaded = service_desk_server()
        .organizations()
        .get_organization(id.to_string())
        .send()
        .await
        .expect("an organization reads back by id");

    assert_eq!(loaded.id.as_deref(), Some(id.to_string().as_str()), "the organization read back is the one asked for");

    service_desk_server()
        .organizations()
        .delete_organization(id.to_string())
        .send()
        .await
        .expect("an organization can be removed");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn adds_lists_and_removes_the_users_of_an_organization() {
    if !service_desk_licensed().await {
        return;
    }

    let id = create_organization("organization with users").await;

    service_desk_server()
        .organizations()
        .add_users_to_organization(id.to_string())
        .users_organization_update(UsersOrganizationUpdate { usernames: Some(vec!["admin".to_owned()]) })
        .send()
        .await
        .expect("an organization accepts a user");

    let users = service_desk_server()
        .organizations()
        .get_users_in_organization(id.to_string())
        .send()
        .await
        .expect("an organization lists its users");

    assert!(!users.values.is_empty(), "the user just added is in the organization");

    service_desk_server()
        .organizations()
        .remove_users_from_organization(id.to_string())
        .users_organization_update(UsersOrganizationUpdate { usernames: Some(vec!["admin".to_owned()]) })
        .send()
        .await
        .expect("a user can be removed from an organization");

    service_desk_server().organizations().delete_organization(id.to_string()).send().await.expect("cleanup");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn attaches_an_organization_to_a_service_desk_and_detaches_it() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let id = create_organization("desk organization").await;

    service_desk_server()
        .service_desk_organizations()
        .add_organization(project.service_desk_id.to_string())
        .organization_service_desk_update(OrganizationServiceDeskUpdate { organization_id: Some(id) })
        .send()
        .await
        .expect("a service desk accepts an organization");

    let attached = service_desk_server()
        .service_desk_organizations()
        .get_service_desk_organizations(project.service_desk_id.to_string())
        .send()
        .await
        .expect("a service desk lists the organizations attached to it");

    assert!(
        attached.values.iter().any(|organization| organization.id.as_deref() == Some(id.to_string().as_str())),
        "the organization just attached is among them",
    );

    service_desk_server()
        .service_desk_organizations()
        .remove_organization(project.service_desk_id.to_string())
        .organization_service_desk_update(OrganizationServiceDeskUpdate { organization_id: Some(id) })
        .send()
        .await
        .expect("an organization can be detached from a service desk");

    service_desk_server().organizations().delete_organization(id.to_string()).send().await.expect("cleanup");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn the_cleanup_endpoints_are_declared_and_not_served() {
    if !service_desk_licensed().await {
        return;
    }

    let refused = service_desk_server()
        .organizations()
        .preview_clean_up_organizations()
        .send()
        .await
        .expect_err("Service Management 10.3 does not route the organization cleanup the document declares");

    assert!(
        refused.status().is_some_and(|status| status >= 400),
        "the refusal is typed rather than a parse failure: the instance answers 500 wrapping its own 404, which is \
         what an endpoint the document declares and the build does not carry looks like",
    );
}

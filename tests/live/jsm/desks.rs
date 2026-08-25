use jira::service_desk_server::{CustomerCreate, ServiceDeskCustomerAdd};

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::{run_id, service_desk_server};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_service_desks_on_the_instance() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;

    let desks =
        service_desk_server().service_desks().get_service_desks().send().await.expect("the instance lists its desks");

    assert!(
        desks.values.iter().any(|desk| desk.project_key.as_deref() == Some(project.project_key.as_str())),
        "the project the fixtures made is served as a service desk",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn loads_one_service_desk_by_id() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;

    let desk = service_desk_server()
        .service_desks()
        .get_service_desk_by_id(project.service_desk_id.to_string())
        .send()
        .await
        .expect("a service desk reads back by id");

    assert_eq!(
        desk.project_key.as_deref(),
        Some(project.project_key.as_str()),
        "the desk read back is the one asked for"
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_a_customer_and_adds_it_to_the_desk() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let username = format!("jirars-{}-customer", run_id());

    let customer = service_desk_server()
        .customers()
        .create_customer()
        .customer_create(CustomerCreate {
            email: Some(format!("{username}@example.com")),
            full_name: Some(asset_name("customer")),
        })
        .send()
        .await
        .expect("the instance accepts a customer");

    let name = customer.name.clone().expect("a created customer carries a name");

    service_desk_server()
        .customers()
        .add_customers(project.service_desk_id.to_string())
        .service_desk_customer_add(ServiceDeskCustomerAdd { usernames: Some(vec![name]) })
        .send()
        .await
        .expect("a customer can be added to a service desk");
}

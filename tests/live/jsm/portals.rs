use super::fixtures::{service_desk_licensed, service_desk_project};
use crate::harness::service_desk_server;

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_portals_and_reads_one_by_project_and_by_id() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;

    let portals = service_desk_server().portals().get_portals().send().await.expect("the instance lists its portals");

    assert!(!portals.values.is_empty(), "a service desk project brings a portal with it");

    let by_project = service_desk_server()
        .portals()
        .get_portal_by_project_key(project.project_key.clone())
        .send()
        .await
        .expect("a portal reads back by the key of the project it serves");

    let id = by_project.id.expect("a portal carries an id");

    let by_id =
        service_desk_server().portals().get_portal(id.to_string()).send().await.expect("a portal reads back by id");

    assert_eq!(by_id.id, Some(id), "the portal read back is the one the project named");
}

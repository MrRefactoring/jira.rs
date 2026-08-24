//! What an instance is configured with rather than what it holds: status types, and the counts it reports.

use jira::assets_server::StatusType;

use super::fixtures::{asset_name, fixtures};
use crate::harness::{ResourceTracker, assets_server};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_status_types_the_instance_ships_with() {
    let statuses = assets_server().status_types().find_status_types().send().await.expect("the status types list");

    assert!(!statuses.is_empty(), "an instance ships status types of its own");
    assert!(statuses.iter().all(|status| status.name.is_some()), "every status type is named");
}

/// The name is capped at thirty characters by Assets, which the run id already spends half of.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_loads_updates_and_deletes_a_status_type() {
    let mut tracker = ResourceTracker::new();

    let created = assets_server()
        .status_types()
        .store_status_type()
        .status_type(StatusType {
            name: Some(asset_name("status")),
            category: Some(1),
            description: Some("Created and removed by one test.".to_owned()),
            ..StatusType::default()
        })
        .send()
        .await
        .expect("Assets accepts a status type");

    let id = created.id.expect("a created status type carries an id");

    tracker
        .defer(move || async move { assets_server().status_types().delete_status_type(id.to_string()).send().await });

    let loaded = assets_server()
        .status_types()
        .get_status_type(id.to_string())
        .send()
        .await
        .expect("a status type reads back by id");

    assert_eq!(loaded.id, Some(id), "the status type read back is the one that was made");

    let updated = assets_server()
        .status_types()
        .update_status_type(id.to_string())
        .body(StatusType { name: Some(asset_name("renamed")), category: Some(2), ..StatusType::default() })
        .send()
        .await
        .expect("a status type can be rewritten");

    assert_eq!(updated.category, Some(2), "the category the body carried is the category that stuck");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_what_each_schema_holds() {
    let fixtures = fixtures().await;

    let analytics = assets_server()
        .analytics()
        .get_schema_analytics()
        .send()
        .await
        .expect("the instance reports its schema counts");

    assert!(
        analytics.iter().any(|entry| entry.schema_id == Some(fixtures.schema_id)),
        "the schema the fixtures made is among the schemas counted",
    );
}

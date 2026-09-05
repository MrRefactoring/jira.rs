//! The objects an Assets instance is for, and everything hung off one.
//!
//! The fixture object is read here and written to; anything these tests create for themselves they also remove, so a
//! developer iterating on one file does not accumulate a schema full of debris.

use std::collections::HashMap;

use jira::assets_server::{
    AssetObjectIn, ObjectAttributeIn, ObjectAttributeValueIn, ObjectFilters, ObjectIQLFilterParam,
};
use serde_json::json;

use super::fixtures::{asset_name, create_object, fixtures};
use crate::harness::{ResourceTracker, assets_server};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn loads_the_object_the_fixtures_made() {
    let fixtures = fixtures().await;

    let object = assets_server()
        .objects()
        .load_object(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object reads back by id");

    assert_eq!(object.id, Some(fixtures.object_id), "the object read back is the one asked for");
    assert_eq!(object.object_key.as_deref(), Some(fixtures.object_key.as_str()), "and it carries the key it was given");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_attributes_of_an_object() {
    let fixtures = fixtures().await;

    let attributes = assets_server()
        .objects()
        .find_object_attributes(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object lists its attributes");

    assert!(!attributes.is_empty(), "an object created with a name has attributes");
    assert!(attributes.iter().all(|attribute| attribute.id.is_some()), "every attribute is addressable by an id");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_the_history_of_an_object() {
    let fixtures = fixtures().await;

    let history = assets_server()
        .objects()
        .find_object_history(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object reports its history");

    assert!(
        history.iter().all(|entry| entry.object_id == Some(fixtures.object_id)),
        "every entry belongs to the object it was asked of",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_what_an_object_references() {
    let fixtures = fixtures().await;

    let references = assets_server()
        .objects()
        .find_object_reference_info(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object reports what it references");

    assert!(
        references.iter().all(|entry| entry.object_type.is_some()),
        "a reference names the object type on the other end of it",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_the_jira_issues_connected_to_an_object() {
    let fixtures = fixtures().await;

    let tickets = assets_server()
        .connected_tickets()
        .find_object_tickets(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object reports the issues connected to it");

    assert!(tickets.tickets.is_some(), "the answer carries a ticket list, empty or not");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_updates_and_deletes_an_object() {
    let fixtures = fixtures().await;
    let mut tracker = ResourceTracker::new();

    let created = create_object(&mut tracker, "created").await;
    let id = created.id.expect("a created object carries an id");

    let renamed = assets_server()
        .objects()
        .update_object(id.to_string())
        .asset_object_in(AssetObjectIn {
            object_type_id: fixtures.object_type_id,
            attributes: vec![ObjectAttributeIn {
                object_type_attribute_id: Some(fixtures.name_attribute_id),
                object_attribute_values: vec![ObjectAttributeValueIn { value: asset_name("renamed") }],
                ..ObjectAttributeIn::default()
            }],
        })
        .send()
        .await
        .expect("an object can be rewritten");

    assert!(
        renamed.label.as_deref().is_some_and(|label| label.contains("renamed")),
        "the label follows the name attribute the update carried",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn archives_an_object_and_restores_it() {
    let fixtures = fixtures().await;
    let mut tracker = ResourceTracker::new();

    let created = create_object(&mut tracker, "archived").await;
    let id = created.id.expect("a created object carries an id");

    assets_server().objects().archive_object(id.to_string()).send().await.expect("an object can be archived");

    let archived = assets_server()
        .objects()
        .get_archived_objects()
        .object_schema_id(fixtures.schema_id.to_string())
        .send()
        .await
        .expect("the schema lists what has been archived in it");

    assert!(
        archived.results.is_some_and(|results| results.iter().any(|entry| entry.id == id)),
        "the object just archived is among them",
    );

    assets_server().objects().restore_object(id.to_string()).send().await.expect("an archived object can be restored");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn archives_in_bulk_by_key_and_restores_by_id() {
    let mut tracker = ResourceTracker::new();

    let created = create_object(&mut tracker, "bulk archived").await;
    let id = created.id.expect("a created object carries an id");
    let key = created.object_key.expect("a created object carries a key");

    assets_server()
        .objects()
        .archive_objects_by_keys()
        .body([key.clone()])
        .send()
        .await
        .expect("objects can be archived by key");

    assets_server().objects().restore_objects_by_ids().body([id]).send().await.expect("objects can be restored by id");

    assets_server()
        .objects()
        .restore_objects_by_keys()
        .body([key])
        .send()
        .await
        .expect("objects can be restored by key");

    tracker.cleanup().await;
}

/// `ql_query_search` is a flag rather than the query — the query goes in `ql_query_params`.
///
/// The document has this right and the shape reads backwards, so it is worth pinning: passing the query where the
/// flag goes earns a 400 about deserialising a Boolean.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn archives_by_filter_and_restores_by_filter() {
    let fixtures = fixtures().await;
    let mut tracker = ResourceTracker::new();

    let created = create_object(&mut tracker, "filtered").await;
    let key = created.object_key.expect("a created object carries a key");

    let archiving = assets_server()
        .objects()
        .archive_objects_by_filter()
        .type_id(fixtures.object_type_id.to_string())
        .object_filters(ObjectFilters {
            object_schema_id: Some(fixtures.schema_id),
            ql_query_search: Some(true),
            ql_query_params: Some(ObjectIQLFilterParam {
                ql_query: Some(format!("objectKey = \"{key}\"")),
                ..ObjectIQLFilterParam::default()
            }),
            ..ObjectFilters::default()
        })
        .send()
        .await
        .expect("objects can be archived by filter");

    let category = archiving.category.clone().expect("an archiving task names its category");
    let resource_id = archiving.resource_id.clone().expect("an archiving task names the resource it works on");

    assert_eq!(category, "archive-objects", "the task is the archiving one rather than something else queued");

    let progress = assets_server()
        .progress()
        .get_progress_for_category_and_resource_id(resource_id.clone(), category)
        .send()
        .await
        .expect("a queued task reports its progress");

    assert_eq!(progress.resource_id, Some(resource_id), "the progress read back is that of the task just started");

    assets_server()
        .objects()
        .restore_objects_by_filter()
        .object_schema_id(fixtures.schema_id.to_string())
        .send()
        .await
        .expect("objects can be restored by filter");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn finds_objects_by_aql() {
    let fixtures = fixtures().await;

    let found = assets_server()
        .aql()
        .find_objects()
        .ql_query(format!("objectSchemaId = {}", fixtures.schema_id))
        .send()
        .await
        .expect("AQL is accepted");

    assert!(
        found.object_entries.is_some_and(|entries| entries.iter().any(|entry| entry.id == Some(fixtures.object_id))),
        "the fixture object is among what the query found",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn sets_reports_and_clears_the_import_source_of_an_object() {
    let fixtures = fixtures().await;

    let body: HashMap<String, serde_json::Value> =
        [("objectIds".to_owned(), json!([fixtures.object_id])), ("importSource".to_owned(), json!("jira-rs"))]
            .into_iter()
            .collect();

    assets_server()
        .objects()
        .bulk_set_object_import_source()
        .body(body)
        .send()
        .await
        .expect("an import source can be set in bulk");

    assets_server()
        .objects()
        .get_object_import_source(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an object reports its import source");

    assets_server()
        .objects()
        .clear_object_import_source(fixtures.object_id.to_string())
        .send()
        .await
        .expect("an import source can be cleared");
}

/// The navigator list, which takes the object type and the schema together rather than one filtered by the other.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn finds_objects_through_the_navigator_list() {
    let fixtures = fixtures().await;

    let found = assets_server()
        .objects()
        .find_object()
        .object_iql_filter_param(ObjectIQLFilterParam {
            object_type_id: Some(fixtures.object_type_id),
            object_schema_id: Some(fixtures.schema_id),
            page: Some(1),
            results_per_page: Some(25),
            include_attributes: Some(true),
            ..ObjectIQLFilterParam::default()
        })
        .send()
        .await
        .expect("the navigator list is accepted");

    assert!(
        found.object_entries.is_some_and(|entries| entries.iter().any(|entry| entry.id == Some(fixtures.object_id))),
        "the fixture object is on the page the navigator returned",
    );
}

//! The structure an Assets object lives in: the schema, its object types, and the attributes those declare.
//!
//! The fixture schema is read here. Anything written to is made and removed by the test that needs it, so a failure
//! leaves the fixtures intact for the files that run after.

use jira::assets_server::{
    JSTreePosition, ObjectAttributeIn, ObjectAttributeValueIn, ObjectSchema, ObjectSchemaIn, ObjectTypeAttributeIn,
    ObjectTypeIn,
};

use super::fixtures::{asset_name, fixtures, schema_key};
use crate::harness::{ResourceTracker, assets_server};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_schemas_on_the_instance() {
    let fixtures = fixtures().await;

    let schemas = assets_server().object_schemas().find_schemas().send().await.expect("the instance lists its schemas");

    assert!(
        schemas.objectschemas.is_some_and(|listed| listed.iter().any(|schema| schema.id == Some(fixtures.schema_id))),
        "the schema the fixtures made is among them",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn loads_one_schema() {
    let fixtures = fixtures().await;

    let schema = assets_server()
        .object_schemas()
        .load_schema(fixtures.schema_id.to_string())
        .send()
        .await
        .expect("a schema reads back by id");

    assert_eq!(schema.id, Some(fixtures.schema_id), "the schema read back is the one asked for");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_object_types_of_a_schema_flat() {
    let fixtures = fixtures().await;

    let types = assets_server()
        .object_schemas()
        .find_object_type_flat_list(fixtures.schema_id.to_string())
        .send()
        .await
        .expect("a schema lists its object types");

    assert!(
        types.iter().any(|object_type| object_type.id == Some(fixtures.object_type_id)),
        "the object type the fixtures made is among them",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_updates_and_deletes_a_schema() {
    let mut tracker = ResourceTracker::new();

    let created = assets_server()
        .object_schemas()
        .create_schema()
        .object_schema_in(ObjectSchemaIn {
            name: asset_name("spare schema"),
            object_schema_key: schema_key("spare schema"),
            description: Some("Created and removed by one test.".to_owned()),
        })
        .send()
        .await
        .expect("Assets accepts a second schema");

    let id = created.id.expect("a created schema carries an id");

    tracker.defer(move || async move {
        assets_server().object_schemas().delete_schema(id.to_string()).send().await.map(drop)
    });

    let updated = assets_server()
        .object_schemas()
        .update_schema(id.to_string())
        .body(ObjectSchema { description: Some("Updated by one test.".to_owned()), ..created })
        .send()
        .await
        .expect("a schema can be rewritten");

    assert_eq!(
        updated.description.as_deref(),
        Some("Updated by one test."),
        "the description the body carried is the description that stuck",
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn loads_one_object_type() {
    let fixtures = fixtures().await;

    let object_type = assets_server()
        .object_types()
        .load_object_type(fixtures.object_type_id.to_string())
        .send()
        .await
        .expect("an object type reads back by id");

    assert_eq!(object_type.id, Some(fixtures.object_type_id), "the object type read back is the one asked for");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_the_attributes_of_an_object_type() {
    let fixtures = fixtures().await;

    let attributes = assets_server()
        .object_types()
        .find_object_type_attributes(fixtures.object_type_id.to_string())
        .send()
        .await
        .expect("an object type lists the attributes it declares");

    assert!(
        attributes.iter().any(|attribute| attribute.id == Some(fixtures.name_attribute_id)),
        "the Name attribute the fixtures write through is among them",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_updates_repositions_and_deletes_an_object_type() {
    let fixtures = fixtures().await;
    let mut tracker = ResourceTracker::new();

    let created = assets_server()
        .object_types()
        .create_object_type()
        .object_type_in(ObjectTypeIn {
            name: Some(asset_name("spare type")),
            object_schema_id: Some(fixtures.schema_id),
            icon_id: Some(fixtures.icon_id),
            description: Some("Created and removed by one test.".to_owned()),
            ..ObjectTypeIn::default()
        })
        .send()
        .await
        .expect("the schema accepts a second object type");

    let id = created.id.expect("a created object type carries an id");

    tracker.defer(move || async move {
        assets_server().object_types().delete_object_type(id.to_string()).send().await.map(drop)
    });

    let updated = assets_server()
        .object_types()
        .update_object_type(id.to_string())
        .body(ObjectTypeIn {
            name: Some(asset_name("renamed type")),
            object_schema_id: Some(fixtures.schema_id),
            icon_id: Some(fixtures.icon_id),
            ..ObjectTypeIn::default()
        })
        .send()
        .await
        .expect("an object type can be rewritten");

    assert!(
        updated.name.as_deref().is_some_and(|name| name.contains("renamed type")),
        "the name the body carried is the name that stuck",
    );

    // The tree takes a position relative to a sibling, so the fixture type is what this one is placed against.
    assets_server()
        .object_types()
        .change_order_object_type(id.to_string())
        .js_tree_position(JSTreePosition { to_object_type_id: Some(fixtures.object_type_id), position: Some(0) })
        .send()
        .await
        .expect("an object type can be moved within the tree");

    tracker.cleanup().await;
}

/// Setting an attribute on an object one attribute at a time, rather than through the whole object.
///
/// The endpoint is `create`, and it updates: the pair of an object and an object type attribute is the identity, so
/// calling it twice on the same pair replaces the value instead of adding a second one.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn writes_one_attribute_of_an_object_on_its_own() {
    let fixtures = fixtures().await;

    let written = assets_server()
        .object_attributes()
        .create_object_attribute()
        .object_attribute_in(ObjectAttributeIn {
            object_id: Some(fixtures.object_id),
            object_type_attribute_id: Some(fixtures.name_attribute_id),
            object_attribute_values: vec![ObjectAttributeValueIn { value: asset_name("written directly") }],
            ..ObjectAttributeIn::default()
        })
        .send()
        .await
        .expect("one attribute of an object can be written on its own");

    assert_eq!(
        written.object_type_attribute_id,
        Some(fixtures.name_attribute_id),
        "the value landed on the attribute it was addressed to",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn creates_updates_and_deletes_an_attribute_on_an_object_type() {
    let fixtures = fixtures().await;
    let mut tracker = ResourceTracker::new();

    let created = assets_server()
        .object_type_attributes()
        .store_object_type_attribute(fixtures.object_type_id.to_string())
        .object_type_attribute_in(ObjectTypeAttributeIn {
            name: Some(asset_name("spare attribute")),
            default_type_id: Some(0),
            description: Some("Created and removed by one test.".to_owned()),
            ..ObjectTypeAttributeIn::default()
        })
        .send()
        .await
        .expect("an object type accepts a second attribute");

    let id = created.id.expect("a created attribute carries an id");

    tracker.defer(move || async move {
        assets_server().object_type_attributes().delete_object_type_attribute(id.to_string()).send().await
    });

    let updated = assets_server()
        .object_type_attributes()
        .update_object_type_attribute(fixtures.object_type_id.to_string(), id.to_string())
        .body(ObjectTypeAttributeIn {
            name: Some(asset_name("renamed attribute")),
            default_type_id: Some(0),
            ..ObjectTypeAttributeIn::default()
        })
        .send()
        .await
        .expect("an attribute can be rewritten");

    assert!(
        updated.name.as_deref().is_some_and(|name| name.contains("renamed attribute")),
        "the name the body carried is the name that stuck",
    );

    tracker.cleanup().await;
}

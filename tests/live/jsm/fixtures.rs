//! Everything the Service Management suites need to exist before they run.
//!
//! A fresh Assets holds no schema at all, and most of the surface is unreachable until one is there to point at: an
//! object schema with one object type, one attribute and one object opens the objects, attributes, comments,
//! attachments and history endpoints together. Built once for the whole binary, because every suite reads it and
//! none of them may remove it.

use jira::assets_server::{
    AssetObjectIn, ObjectAttributeIn, ObjectAttributeValueIn, ObjectSchemaIn, ObjectTypeAttribute, ObjectTypeIn,
};
use tokio::sync::OnceCell;

use crate::harness::{RESOURCE_MARKER, ResourceTracker, assets_server, run_id, run_suffix, service_desk_server};

/// The world the suites run in, made once.
pub struct Fixtures {
    pub schema_id: i64,
    pub object_type_id: i64,
    /// The `Name` attribute every object type is created with, and the only one an object can be given a value for.
    pub name_attribute_id: i64,
    pub object_id: i64,
    pub object_key: String,
    /// A global icon, which Assets ships a few dozen of and every object type needs one of.
    pub icon_id: i64,
}

/// A run-scoped name Assets will accept.
///
/// [`crate::harness::test_name`] cannot be used here: it brackets the run id with a colon, and Assets rejects `=;:?."`
/// in the name of a schema, an object type or an object — a 400 calling them reserved characters, which is a good deal
/// less obvious arriving from a fixture than from a test.
pub fn asset_name(label: &str) -> String {
    format!("{RESOURCE_MARKER}-{} {label}", run_id())
}

/// A schema key: uppercase letters only, and short.
///
/// Assets builds every object key out of it — `JRSABCDEFG-1` — so it has to be unique on the instance, and the
/// instance outlives a run: the fixture schema is never removed, so the next run must not ask for the key the last one
/// took. The label separates the fixture schema from a schema a single test makes for itself.
pub fn schema_key(label: &str) -> String {
    format!("JRS{}", run_suffix(label, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ", 7))
}

/// The fixtures, built on first use and shared by every suite.
pub async fn fixtures() -> &'static Fixtures {
    static FIXTURES: OnceCell<Fixtures> = OnceCell::const_new();

    FIXTURES.get_or_init(build).await
}

/// Whether Service Desk answers at all on this instance.
///
/// Assets ships with the image and its REST module does not check for a seat, so a Jira Software timebomb opens
/// `/rest/assets/1.0` completely while every `/rest/servicedeskapi/` endpoint answers 403. Asked once, so the Service
/// Desk suite can stand down visibly instead of failing over a licence.
pub async fn service_desk_licensed() -> bool {
    static LICENSED: OnceCell<bool> = OnceCell::const_new();

    *LICENSED
        .get_or_init(|| async {
            match service_desk_server().info().get_info().send().await {
                Ok(info) => info.is_licensed_for_use == Some(true),
                Err(_) => false,
            }
        })
        .await
}

/// The first attribute of an object type a caller may write to. Assets makes Key, Created and Updated itself.
fn editable_attribute_id(attributes: &[ObjectTypeAttribute]) -> i64 {
    attributes
        .iter()
        .find(|attribute| attribute.editable == Some(true))
        .and_then(|attribute| attribute.id)
        .expect("an object type is created with an editable Name attribute")
}

async fn build() -> Fixtures {
    let assets = assets_server();

    let icons = assets.icons().find_global_icons().send().await.expect("the instance lists the icons it ships");
    let icon_id = icons
        .first()
        .and_then(|icon| icon.id)
        .expect("the instance ships global icons, without which no object type can be created");

    let schema = assets
        .object_schemas()
        .create_schema()
        .object_schema_in(ObjectSchemaIn {
            name: asset_name("schema"),
            object_schema_key: schema_key("fixtures"),
            description: Some("Created by the jira live suite.".to_owned()),
        })
        .send()
        .await
        .expect("Assets accepts an object schema");

    let schema_id = schema.id.expect("a created schema carries an id");

    let object_type = assets
        .object_types()
        .create_object_type()
        .object_type_in(ObjectTypeIn {
            name: Some(asset_name("object type")),
            object_schema_id: Some(schema_id),
            icon_id: Some(icon_id),
            description: Some("Created by the jira live suite.".to_owned()),
            ..ObjectTypeIn::default()
        })
        .send()
        .await
        .expect("the schema accepts an object type");

    let object_type_id = object_type.id.expect("a created object type carries an id");

    let attributes = assets
        .object_types()
        .find_object_type_attributes(object_type_id.to_string())
        .send()
        .await
        .expect("an object type lists the attributes it declares");

    let name_attribute_id = editable_attribute_id(&attributes);

    let object = assets
        .objects()
        .create_object()
        .asset_object_in(AssetObjectIn {
            object_type_id,
            attributes: vec![ObjectAttributeIn {
                object_type_attribute_id: Some(name_attribute_id),
                object_attribute_values: vec![ObjectAttributeValueIn { value: asset_name("object") }],
                ..ObjectAttributeIn::default()
            }],
        })
        .send()
        .await
        .expect("the object type accepts an object");

    Fixtures {
        schema_id,
        object_type_id,
        name_attribute_id,
        object_id: object.id.expect("a created object carries an id"),
        object_key: object.object_key.expect("a created object carries a key"),
        icon_id,
    }
}

/// An object of the calling test's own, and the removal of it.
pub async fn create_object(tracker: &mut ResourceTracker, label: &str) -> jira::assets_server::AssetObject {
    let fixtures = fixtures().await;

    let created = assets_server()
        .objects()
        .create_object()
        .asset_object_in(AssetObjectIn {
            object_type_id: fixtures.object_type_id,
            attributes: vec![ObjectAttributeIn {
                object_type_attribute_id: Some(fixtures.name_attribute_id),
                object_attribute_values: vec![ObjectAttributeValueIn { value: asset_name(label) }],
                ..ObjectAttributeIn::default()
            }],
        })
        .send()
        .await
        .expect("the object type accepts an object");

    let id = created.id.expect("a created object carries an id");

    tracker
        .defer(move || async move { assets_server().objects().delete_object(id.to_string()).send().await.map(drop) });

    created
}

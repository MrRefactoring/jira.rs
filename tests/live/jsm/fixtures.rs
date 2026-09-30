use jira::assets_server::{
    AssetObjectIn, ObjectAttributeIn, ObjectAttributeValueIn, ObjectSchemaIn, ObjectTypeAttribute, ObjectTypeIn,
};
use tokio::sync::OnceCell;

use crate::harness::{
    RESOURCE_MARKER, ResourceTracker, assets_server, jsm_platform, require_jsm_env, run_id, run_suffix,
    service_desk_server,
};

pub struct Fixtures {
    pub schema_id: i64,
    pub object_type_id: i64,
    pub name_attribute_id: i64,
    pub object_id: i64,
    pub object_key: String,
    pub icon_id: i64,
}

pub fn jsm_username() -> String {
    require_jsm_env().username
}

pub fn asset_name(label: &str) -> String {
    format!("{RESOURCE_MARKER}-{} {label}", run_id())
}

pub fn schema_key(label: &str) -> String {
    format!("JRS{}", run_suffix(label, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ", 7))
}

pub struct ServiceDeskProject {
    pub service_desk_id: i64,
    pub project_key: String,
}

const SERVICE_DESK_TEMPLATE: &str = "com.atlassian.servicedesk:basic-service-desk-project";

pub async fn service_desk_project() -> &'static ServiceDeskProject {
    static PROJECT: OnceCell<ServiceDeskProject> = OnceCell::const_new();

    PROJECT.get_or_init(build_service_desk_project).await
}

async fn build_service_desk_project() -> ServiceDeskProject {
    let project_key = schema_key("service desk");

    jsm_platform()
        .projects()
        .create_project(jira::server::ProjectInput {
            key: Some(project_key.clone()),
            name: Some(asset_name("service desk")),
            lead: Some(jsm_username()),
            project_type_key: Some("service_desk".to_owned()),
            project_template_key: Some(SERVICE_DESK_TEMPLATE.to_owned()),
            ..jira::server::ProjectInput::default()
        })
        .send()
        .await
        .expect("a licensed instance accepts a service desk project");

    let desks = service_desk_server()
        .service_desks()
        .get_service_desks()
        .send()
        .await
        .expect("the instance lists its service desks");

    let service_desk_id = desks
        .values
        .iter()
        .find(|desk| desk.project_key.as_deref() == Some(project_key.as_str()))
        .and_then(|desk| desk.id.as_ref())
        .and_then(|id| id.parse::<i64>().ok())
        .expect("the project just created is served as a service desk");

    ServiceDeskProject { service_desk_id, project_key }
}

pub async fn fixtures() -> &'static Fixtures {
    static FIXTURES: OnceCell<Fixtures> = OnceCell::const_new();

    FIXTURES.get_or_init(build).await
}

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

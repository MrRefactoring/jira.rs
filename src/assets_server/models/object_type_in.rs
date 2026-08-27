// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectTypeIn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconId", default, skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<i64>,
    #[serde(rename = "objectSchemaId", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_id: Option<i64>,
    #[serde(rename = "parentObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub parent_object_type_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(rename = "abstractObjectType", default, skip_serializing_if = "Option::is_none")]
    pub abstract_object_type: Option<bool>,
}

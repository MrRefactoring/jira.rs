// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectSchemaKey", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    #[serde(rename = "objectCount", default, skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i64>,
    #[serde(rename = "archivedObjectCount", default, skip_serializing_if = "Option::is_none")]
    pub archived_object_count: Option<i64>,
    #[serde(rename = "objectTypeCount", default, skip_serializing_if = "Option::is_none")]
    pub object_type_count: Option<i64>,
}

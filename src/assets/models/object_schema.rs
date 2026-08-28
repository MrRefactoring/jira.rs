// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSchema {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "objectSchemaKey")]
    pub object_schema_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Always 'Ok'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created: String,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub updated: String,
    #[serde(rename = "objectCount")]
    pub object_count: i64,
    #[serde(rename = "objectTypeCount")]
    pub object_type_count: i64,
    #[serde(rename = "canManage", default, skip_serializing_if = "Option::is_none")]
    pub can_manage: Option<bool>,
}

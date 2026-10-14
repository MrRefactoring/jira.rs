// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSchemaCdmDataTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opinionated: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSchemaCdmData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ObjectSchemaCdmDataTypes>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSchema {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
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
        serialize_with = "crate::core::serialize_datetime_rfc3339"
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
        serialize_with = "crate::core::serialize_datetime_rfc3339"
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
    #[serde(rename = "atlassianTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub atlassian_template_id: Option<String>,
    #[serde(rename = "cdmData", default, skip_serializing_if = "Option::is_none")]
    pub cdm_data: Option<ObjectSchemaCdmData>,
    #[serde(rename = "crossSchemaReferencingAllowed", default, skip_serializing_if = "Option::is_none")]
    pub cross_schema_referencing_allowed: Option<bool>,
}

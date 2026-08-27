// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The Assets object type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectType {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub icon: Icon,
    pub position: i64,
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created: String,
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub updated: String,
    #[serde(rename = "objectCount")]
    pub object_count: i64,
    /// The id of the parent object type
    #[serde(rename = "parentObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub parent_object_type_id: Option<i64>,
    /// The type of the attribute
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(rename = "objectSchemaId")]
    pub object_schema_id: String,
    /// Describes if this object type is configured for inheritance i.e. it's children inherits the attributes of this object type
    pub inherited: bool,
    #[serde(rename = "abstractObjectType")]
    pub abstract_object_type: bool,
    /// Describes if this object types parent is inherited i.e. this object type has attributes that are inherited from one or more parents
    #[serde(rename = "parentObjectTypeInherited")]
    pub parent_object_type_inherited: bool,
}

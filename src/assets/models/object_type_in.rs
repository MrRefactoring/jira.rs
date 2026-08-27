// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The Assets object type input used for creating object types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectTypeIn {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconId")]
    pub icon_id: String,
    #[serde(rename = "objectSchemaId")]
    pub object_schema_id: String,
    /// The id of the parent object type
    #[serde(rename = "parentObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub parent_object_type_id: Option<String>,
    /// Describes if this object type is configured for inheritance i.e. it's children inherits the attributes of this object type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(rename = "abstractObjectType", default, skip_serializing_if = "Option::is_none")]
    pub abstract_object_type: Option<bool>,
}

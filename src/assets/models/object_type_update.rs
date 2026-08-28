// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The Assets object type input used for updating object types
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectTypeUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconId", default, skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<String>,
    /// Describes if this object type is configured for inheritance i.e. it's children inherits the attributes of this object type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(rename = "abstractObjectType", default, skip_serializing_if = "Option::is_none")]
    pub abstract_object_type: Option<bool>,
}

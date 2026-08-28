// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectTypePosition {
    /// The desired new parent of the object type
    #[serde(rename = "toObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub to_object_type_id: Option<String>,
    /// The preffered position
    pub position: i64,
}

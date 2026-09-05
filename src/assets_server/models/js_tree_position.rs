// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JSTreePosition {
    #[serde(rename = "toObjectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub to_object_type_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

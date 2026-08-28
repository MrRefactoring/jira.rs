// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueLinkTypeOrderUpdateRequest {
    #[serde(rename = "newPosition", default, skip_serializing_if = "Option::is_none")]
    pub new_position: Option<i64>,
}

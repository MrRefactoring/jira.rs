// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SprintSwap {
    #[serde(rename = "sprintToSwapWith", default, skip_serializing_if = "Option::is_none")]
    pub sprint_to_swap_with: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnmapSprints {
    #[serde(rename = "sprintIds", default, skip_serializing_if = "Option::is_none")]
    pub sprint_ids: Option<Vec<i64>>,
}

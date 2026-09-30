// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraPriorityField {
    #[serde(rename = "priorityId")]
    pub priority_id: String,
}

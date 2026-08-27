// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectDeleteInstructions {
    #[serde(rename = "grantsToDelete", default, skip_serializing_if = "Option::is_none")]
    pub grants_to_delete: Option<Vec<i64>>,
}

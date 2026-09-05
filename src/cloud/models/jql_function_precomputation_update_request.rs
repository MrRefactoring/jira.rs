// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of pairs (id and value) for precomputation updates.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<JqlFunctionPrecomputationUpdate>>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Operation for a SCIM patch
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationForAScimPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonNode>,
}

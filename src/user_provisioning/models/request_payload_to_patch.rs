// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Request object to patch a scim user
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestPayloadToPatch {
    /// SCIM patch schemas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// SCIM patch operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OperationForAScimPatch>>,
}

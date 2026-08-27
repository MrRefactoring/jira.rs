// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestTypePermissionCheckResponse {
    /// List of request type IDs for which the user has permission to administer.
    #[serde(rename = "canAdminister", default, skip_serializing_if = "Option::is_none")]
    pub can_administer: Option<Vec<i64>>,
    /// List of request type IDs for which the user can create requests.
    #[serde(rename = "canCreateRequest", default, skip_serializing_if = "Option::is_none")]
    pub can_create_request: Option<Vec<i64>>,
}

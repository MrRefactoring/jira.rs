// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantToPermissionInput>>,
    #[serde(rename = "permissionKeys", default, skip_serializing_if = "Option::is_none")]
    pub permission_keys: Option<Vec<String>>,
}

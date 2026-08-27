// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of all permission schemes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionSchemes {
    /// Permission schemes list.
    #[serde(rename = "permissionSchemes", default, skip_serializing_if = "Option::is_none")]
    pub permission_schemes: Option<Vec<PermissionScheme>>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details for permissions of shareable entities
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PermissionDetails {
    /// The edit permissions for the shareable entities.
    #[serde(rename = "editPermissions")]
    pub edit_permissions: Vec<SharePermission>,
    /// The share permissions for the shareable entities.
    #[serde(rename = "sharePermissions")]
    pub share_permissions: Vec<SharePermission>,
}

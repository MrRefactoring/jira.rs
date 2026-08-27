// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Allowed action for bulk edit shareable entity
    pub enum BulkEditShareableEntityRequestAction {
        ChangeOwner => "changeOwner",
        ChangePermission => "changePermission",
        AddPermission => "addPermission",
        RemovePermission => "removePermission",
    }
}

/// Details of a request to bulk edit shareable entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditShareableEntityRequest {
    /// Allowed action for bulk edit shareable entity
    pub action: BulkEditShareableEntityRequestAction,
    #[serde(rename = "changeOwnerDetails", default, skip_serializing_if = "Option::is_none")]
    pub change_owner_details: Option<BulkChangeOwnerDetails>,
    /// The id list of shareable entities to be changed.
    #[serde(rename = "entityIds")]
    pub entity_ids: Vec<i64>,
    /// Whether the actions are executed by users with Administer Jira global permission.
    #[serde(rename = "extendAdminPermissions", default, skip_serializing_if = "Option::is_none")]
    pub extend_admin_permissions: Option<bool>,
    #[serde(rename = "permissionDetails", default, skip_serializing_if = "Option::is_none")]
    pub permission_details: Option<PermissionDetails>,
}

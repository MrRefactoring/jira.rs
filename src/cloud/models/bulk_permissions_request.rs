// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of global permissions to look up and project permissions with associated projects and issues to look up.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkPermissionsRequest {
    /// The account ID of a user.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Global permissions to look up.
    #[serde(rename = "globalPermissions", default, skip_serializing_if = "Option::is_none")]
    pub global_permissions: Option<Vec<String>>,
    /// Project permissions with associated projects and issues to look up.
    #[serde(rename = "projectPermissions", default, skip_serializing_if = "Option::is_none")]
    pub project_permissions: Option<Vec<BulkProjectPermissions>>,
}

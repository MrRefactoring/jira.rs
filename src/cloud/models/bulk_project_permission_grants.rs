// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// List of project permissions and the projects and issues those permissions grant access to.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkProjectPermissionGrants {
    /// IDs of the issues the user has the permission for.
    pub issues: Vec<i64>,
    /// A project permission,
    pub permission: String,
    /// IDs of the projects the user has the permission for.
    pub projects: Vec<i64>,
}

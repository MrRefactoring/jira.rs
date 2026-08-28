// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of project permissions and associated issues and projects to look up.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkProjectPermissions {
    /// List of issue IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<i64>>,
    /// List of project permissions.
    pub permissions: Vec<String>,
    /// List of project IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
}

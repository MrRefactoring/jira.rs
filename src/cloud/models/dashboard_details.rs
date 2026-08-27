// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a dashboard.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardDetails {
    /// The description of the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The edit permissions for the dashboard.
    #[serde(rename = "editPermissions")]
    pub edit_permissions: Vec<SharePermission>,
    /// The name of the dashboard.
    pub name: String,
    /// The share permissions for the dashboard.
    #[serde(rename = "sharePermissions")]
    pub share_permissions: Vec<SharePermission>,
}

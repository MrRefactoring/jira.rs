// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Permissions which a user has on a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectPermissions {
    /// Whether the logged user can edit the project.
    #[serde(rename = "canEdit", default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
}

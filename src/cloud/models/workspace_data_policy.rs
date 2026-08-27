// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about data policy.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceDataPolicy {
    /// Whether the workspace contains any content inaccessible to the requesting application.
    #[serde(rename = "anyContentBlocked", default, skip_serializing_if = "Option::is_none")]
    pub any_content_blocked: Option<bool>,
}

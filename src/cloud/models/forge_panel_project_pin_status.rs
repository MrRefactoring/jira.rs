// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The pin status of an issue panel (added by a Forge app) for a single project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForgePanelProjectPinStatus {
    /// The reason the pin status could not be read for the project. Null if the pin status was read successfully.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether the issue panel is currently pinned to the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// The time the issue panel was pinned to the project, in epoch milliseconds.
    #[serde(rename = "pinnedAt", default, skip_serializing_if = "Option::is_none")]
    pub pinned_at: Option<i64>,
    /// The project ID or key supplied in the request.
    #[serde(rename = "projectIdOrKey", default, skip_serializing_if = "Option::is_none")]
    pub project_id_or_key: Option<String>,
}

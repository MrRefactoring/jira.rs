// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an insight workspace ID.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InsightWorkspace {
    /// The workspace ID used as the identifier to access the Insight REST API.
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

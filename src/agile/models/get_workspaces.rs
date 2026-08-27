// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The payload of Operations Workspace Ids.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetWorkspaces {
    /// The IDs of Operations Workspaces that are available to this Jira site.
    #[serde(rename = "workspaceIds")]
    pub workspace_ids: Vec<String>,
}

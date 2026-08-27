// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The payload of linked Security Workspace IDs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLinkedWorkspaces {
    /// The IDs of Security Workspaces that are linked to this Jira site.
    #[serde(rename = "workspaceIds")]
    pub workspace_ids: Vec<String>,
}

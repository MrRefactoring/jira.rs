// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The result of a successful submitOperationsWorkspaces request.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SubmitOperationsWorkspaces {
    /// The IDs of Operations Workspaces that have been linked to the Jira site in this request.
    #[serde(rename = "acceptedWorkspaceIds", default, skip_serializing_if = "Option::is_none")]
    pub accepted_workspace_ids: Option<Vec<String>>,
}

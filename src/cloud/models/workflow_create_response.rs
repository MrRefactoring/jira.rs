// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the created workflows and statuses.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowCreateResponse {
    /// List of created statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<JiraWorkflowStatus>>,
    /// List of created workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<JiraWorkflow>>,
}

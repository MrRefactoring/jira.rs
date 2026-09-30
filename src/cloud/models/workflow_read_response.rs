// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of workflows and related statuses.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowReadResponse {
    /// List of statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<JiraWorkflowStatus>>,
    /// List of workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<JiraWorkflow>>,
}

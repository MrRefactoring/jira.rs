// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowUpdateResponse {
    /// List of updated statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<JiraWorkflowStatus>>,
    /// If there is a [asynchronous task](#async-operations) operation, as a result of this update.
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// List of updated workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<JiraWorkflow>>,
}

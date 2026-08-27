// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A request to read all the workflow history entries for a specific workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowHistoryListRequest {
    /// The id of the workflow to read the history for.
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

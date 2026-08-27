// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The statuses associated with each workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusesPerWorkflow {
    /// The ID of the initial status for the workflow.
    #[serde(rename = "initialStatusId", default, skip_serializing_if = "Option::is_none")]
    pub initial_status_id: Option<String>,
    /// The status IDs associated with the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

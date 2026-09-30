// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The copy workflow payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowCopyRequest {
    /// The description of the new workflow to create. Defaults to an empty description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the workflow to copy.
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    /// The name of the new workflow to create.
    #[serde(rename = "workflowName")]
    pub workflow_name: String,
}

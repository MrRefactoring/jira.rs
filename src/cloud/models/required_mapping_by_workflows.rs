// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The list of required status mappings by workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequiredMappingByWorkflows {
    /// The ID of the source workflow.
    #[serde(rename = "sourceWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub source_workflow_id: Option<String>,
    /// The status IDs requiring mapping.
    #[serde(rename = "statusIds", default, skip_serializing_if = "Option::is_none")]
    pub status_ids: Option<Vec<String>>,
    /// The ID of the target workflow.
    #[serde(rename = "targetWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub target_workflow_id: Option<String>,
}

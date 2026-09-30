// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The status mappings by workflows. Status mappings are required when the new workflow for an issue type doesn't contain all statuses that the old workflow has. Status mappings can be provided by a combination of `statusMappingsByWorkflows` and `statusMappingsByIssueTypeOverride`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MappingsByWorkflow {
    /// The ID of the new workflow.
    #[serde(rename = "newWorkflowId")]
    pub new_workflow_id: String,
    /// The ID of the old workflow.
    #[serde(rename = "oldWorkflowId")]
    pub old_workflow_id: String,
    /// The list of status mappings.
    #[serde(rename = "statusMappings")]
    pub status_mappings: Vec<WorkflowAssociationStatusMapping>,
}

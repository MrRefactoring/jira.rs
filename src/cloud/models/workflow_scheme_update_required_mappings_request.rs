// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The request payload to get the required mappings for updating a workflow scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowSchemeUpdateRequiredMappingsRequest {
    /// The ID of the new default workflow for this workflow scheme. Only used in global-scoped workflow schemes. If it isn't specified, is set to *Jira Workflow (jira)*.
    #[serde(rename = "defaultWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub default_workflow_id: Option<String>,
    /// The ID of the workflow scheme.
    pub id: String,
    /// The new workflow to issue type mappings for this workflow scheme.
    #[serde(rename = "workflowsForIssueTypes")]
    pub workflows_for_issue_types: Vec<WorkflowSchemeAssociation>,
}

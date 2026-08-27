// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowSchemeUpdateRequiredMappingsResponse {
    /// The list of required status mappings by issue type.
    #[serde(rename = "statusMappingsByIssueTypes", default, skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_issue_types: Option<Vec<RequiredMappingByIssueType>>,
    /// The list of required status mappings by workflow.
    #[serde(rename = "statusMappingsByWorkflows", default, skip_serializing_if = "Option::is_none")]
    pub status_mappings_by_workflows: Option<Vec<RequiredMappingByWorkflows>>,
    /// The details of the statuses in the associated workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<StatusMetadata>>,
    /// The statuses associated with each workflow.
    #[serde(rename = "statusesPerWorkflow", default, skip_serializing_if = "Option::is_none")]
    pub statuses_per_workflow: Option<Vec<StatusesPerWorkflow>>,
}

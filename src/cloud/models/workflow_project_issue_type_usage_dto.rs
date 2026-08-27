// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue types associated with the workflow for a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowProjectIssueTypeUsageDTO {
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<WorkflowProjectIssueTypeUsagePage>,
    /// The ID of the project.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The ID of the workflow.
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Projects using the workflow.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowProjectUsageDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<ProjectUsagePage>,
    /// The workflow ID.
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

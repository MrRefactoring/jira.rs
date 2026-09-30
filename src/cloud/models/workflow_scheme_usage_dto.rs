// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Workflow schemes using the workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowSchemeUsageDTO {
    /// The workflow ID.
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowSchemes", default, skip_serializing_if = "Option::is_none")]
    pub workflow_schemes: Option<WorkflowSchemeUsagePage>,
}

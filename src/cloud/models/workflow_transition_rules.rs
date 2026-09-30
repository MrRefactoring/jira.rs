// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A workflow with transition rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRules {
    /// The list of conditions within the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<AppWorkflowTransitionRule>>,
    /// The list of post functions within the workflow.
    #[serde(rename = "postFunctions", default, skip_serializing_if = "Option::is_none")]
    pub post_functions: Option<Vec<AppWorkflowTransitionRule>>,
    /// The list of validators within the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<AppWorkflowTransitionRule>>,
    #[serde(rename = "workflowId")]
    pub workflow_id: WorkflowId,
}

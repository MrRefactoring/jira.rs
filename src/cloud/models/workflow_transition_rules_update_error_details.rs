// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of any errors encountered while updating workflow transition rules for a workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowTransitionRulesUpdateErrorDetails {
    /// A list of transition rule update errors, indexed by the transition rule ID. Any transition rule that appears here wasn't updated.
    #[serde(rename = "ruleUpdateErrors")]
    pub rule_update_errors: std::collections::HashMap<String, serde_json::Value>,
    /// The list of errors that specify why the workflow update failed. The workflow was not updated if the list contains any entries.
    #[serde(rename = "updateErrors")]
    pub update_errors: Vec<String>,
    #[serde(rename = "workflowId")]
    pub workflow_id: WorkflowId,
}

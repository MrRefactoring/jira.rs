// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of workflow transition rules.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowRulesSearchDetails {
    /// List of workflow rule IDs that do not belong to the workflow or can not be found.
    #[serde(rename = "invalidRules", default, skip_serializing_if = "Option::is_none")]
    pub invalid_rules: Option<Vec<String>>,
    /// List of valid workflow transition rules.
    #[serde(rename = "validRules", default, skip_serializing_if = "Option::is_none")]
    pub valid_rules: Option<Vec<WorkflowTransitionRules>>,
    /// The workflow ID.
    #[serde(rename = "workflowEntityId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_entity_id: Option<String>,
}

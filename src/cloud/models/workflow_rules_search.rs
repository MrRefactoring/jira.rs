// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the workflow and its transition rules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowRulesSearch {
    /// Use expand to include additional information in the response. This parameter accepts `transition` which, for each rule, returns information about the transition the rule is assigned to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The list of workflow rule IDs.
    #[serde(rename = "ruleIds")]
    pub rule_ids: Vec<String>,
    /// The workflow ID.
    #[serde(rename = "workflowEntityId")]
    pub workflow_entity_id: String,
}

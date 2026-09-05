// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The configuration of the rule.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowRuleConfiguration {
    /// The ID of the rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The parameters related to the rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The rule key of the rule.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
}

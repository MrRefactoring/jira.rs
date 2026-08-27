// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The rule type.
    pub enum AvailableWorkflowForgeRuleRuleType {
        Condition => "Condition",
        Validator => "Validator",
        Function => "Function",
        Screen => "Screen",
    }
}

/// The Forge provided ecosystem rules available.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableWorkflowForgeRule {
    /// The rule description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique ARI of the forge rule type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The rule name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rule key.
    #[serde(rename = "ruleKey", default, skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    /// The rule type.
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<AvailableWorkflowForgeRuleRuleType>,
}

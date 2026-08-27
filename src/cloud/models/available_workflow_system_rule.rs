// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The rule type.
    pub enum AvailableWorkflowSystemRuleRuleType {
        Condition => "Condition",
        Validator => "Validator",
        Function => "Function",
        Screen => "Screen",
    }
}

/// The Atlassian provided system rules available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableWorkflowSystemRule {
    /// The rule description.
    pub description: String,
    /// List of rules that conflict with this one.
    #[serde(rename = "incompatibleRuleKeys")]
    pub incompatible_rule_keys: Vec<String>,
    /// Whether the rule can be added added to an initial transition.
    #[serde(rename = "isAvailableForInitialTransition")]
    pub is_available_for_initial_transition: bool,
    /// Whether the rule is visible.
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    /// The rule name.
    pub name: String,
    /// The rule key.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
    /// The rule type.
    #[serde(rename = "ruleType")]
    pub rule_type: AvailableWorkflowSystemRuleRuleType,
}

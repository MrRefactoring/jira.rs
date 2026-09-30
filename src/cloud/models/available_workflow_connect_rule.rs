// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The rule type.
    pub enum AvailableWorkflowConnectRuleRuleType {
        Condition => "Condition",
        Validator => "Validator",
        Function => "Function",
        Screen => "Screen",
    }
}

/// The Connect provided ecosystem rules available.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableWorkflowConnectRule {
    /// The add-on providing the rule.
    #[serde(rename = "addonKey", default, skip_serializing_if = "Option::is_none")]
    pub addon_key: Option<String>,
    /// The URL creation path segment defined in the Connect module.
    #[serde(rename = "createUrl", default, skip_serializing_if = "Option::is_none")]
    pub create_url: Option<String>,
    /// The rule description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL edit path segment defined in the Connect module.
    #[serde(rename = "editUrl", default, skip_serializing_if = "Option::is_none")]
    pub edit_url: Option<String>,
    /// The module providing the rule.
    #[serde(rename = "moduleKey", default, skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    /// The rule name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The rule key.
    #[serde(rename = "ruleKey", default, skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
    /// The rule type.
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<AvailableWorkflowConnectRuleRuleType>,
    /// The URL view path segment defined in the Connect module.
    #[serde(rename = "viewUrl", default, skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}

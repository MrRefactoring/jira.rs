// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Trigger configuration for workflow transitions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreviewTrigger {
    /// The ID of the trigger.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the trigger rule.
    #[serde(rename = "ruleKey", default, skip_serializing_if = "Option::is_none")]
    pub rule_key: Option<String>,
}

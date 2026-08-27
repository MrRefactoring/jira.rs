// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A workflow transition rule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowTransitionRule {
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    /// The type of the transition rule.
    pub r#type: String,
}

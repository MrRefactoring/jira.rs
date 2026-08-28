// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A workflow transition rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConnectWorkflowTransitionRule {
    pub configuration: RuleConfiguration,
    /// The ID of the transition rule.
    pub id: String,
    /// The key of the rule, as defined in the Connect app descriptor.
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<WorkflowTransition>,
}

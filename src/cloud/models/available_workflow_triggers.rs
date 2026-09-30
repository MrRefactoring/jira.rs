// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The trigger rules available.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableWorkflowTriggers {
    /// The list of available trigger types.
    #[serde(rename = "availableTypes")]
    pub available_types: Vec<AvailableWorkflowTriggerTypes>,
    /// The rule key of the rule.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
}

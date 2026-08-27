// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
    pub enum ConditionGroupPayloadOperation {
        Any => "ANY",
        All => "ALL",
    }
}

/// The payload for creating a condition group in a workflow
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionGroupPayload {
    /// The nested conditions of the condition group.
    #[serde(rename = "conditionGroup", default, skip_serializing_if = "Option::is_none")]
    pub condition_group: Option<Vec<Box<ConditionGroupPayload>>>,
    /// The rules for this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RulePayload>>,
    /// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<ConditionGroupPayloadOperation>,
}

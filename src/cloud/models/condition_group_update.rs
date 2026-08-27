// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
    pub enum ConditionGroupUpdateOperation {
        Any => "ANY",
        All => "ALL",
    }
}

/// The conditions group associated with the transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionGroupUpdate {
    /// The nested conditions of the condition group.
    #[serde(rename = "conditionGroups", default, skip_serializing_if = "Option::is_none")]
    pub condition_groups: Option<Vec<Box<ConditionGroupUpdate>>>,
    /// The rules for this condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<WorkflowRuleConfiguration>>,
    /// Determines how the conditions in the group are evaluated. Accepts either `ANY` or `ALL`. If `ANY` is used, at least one condition in the group must be true for the group to evaluate to true. If `ALL` is used, all conditions in the group must be true for the group to evaluate to true.
    pub operation: ConditionGroupUpdateOperation,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A collection of transition rules.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowRules {
    #[serde(rename = "conditionsTree", default, skip_serializing_if = "Option::is_none")]
    pub conditions_tree: Option<Box<WorkflowCondition>>,
    /// The workflow post functions.
    #[serde(rename = "postFunctions", default, skip_serializing_if = "Option::is_none")]
    pub post_functions: Option<Vec<WorkflowTransitionRule>>,
    /// The workflow validators.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<WorkflowTransitionRule>>,
}

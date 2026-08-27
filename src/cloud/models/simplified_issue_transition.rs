// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimplifiedIssueTransition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<IssueTransitionStatus>,
    /// The unique ID of the transition.
    #[serde(rename = "transitionId", default, skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<i64>,
    /// The name of the transition.
    #[serde(rename = "transitionName", default, skip_serializing_if = "Option::is_none")]
    pub transition_name: Option<String>,
}

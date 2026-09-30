// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueBulkTransitionForWorkflow {
    /// Indicates whether all the transitions of this workflow are available in the transitions list or not.
    #[serde(rename = "isTransitionsFiltered", default, skip_serializing_if = "Option::is_none")]
    pub is_transitions_filtered: Option<bool>,
    /// List of issue keys from the request which are associated with this workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    /// List of transitions available for issues from the request which are associated with this workflow.
    ///
    ///  **This list includes only those transitions that are common across the issues in this workflow and do not involve any additional field updates.**
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<SimplifiedIssueTransition>>,
}

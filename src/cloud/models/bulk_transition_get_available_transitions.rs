// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Bulk Transition Get Available Transitions Response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkTransitionGetAvailableTransitions {
    /// List of available transitions for bulk transition operation for requested issues grouped by workflow
    #[serde(rename = "availableTransitions", default, skip_serializing_if = "Option::is_none")]
    pub available_transitions: Option<Vec<IssueBulkTransitionForWorkflow>>,
    /// The end cursor for use in pagination.
    #[serde(rename = "endingBefore", default, skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// The start cursor for use in pagination.
    #[serde(rename = "startingAfter", default, skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkTransitionSubmitInput {
    /// List of all the issue IDs or keys that are to be bulk transitioned.
    #[serde(rename = "selectedIssueIdsOrKeys")]
    pub selected_issue_ids_or_keys: Vec<String>,
    /// The ID of the transition that is to be performed on the issues.
    #[serde(rename = "transitionId")]
    pub transition_id: String,
}

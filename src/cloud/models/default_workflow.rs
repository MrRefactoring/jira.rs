// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the default workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultWorkflow {
    /// Whether a draft workflow scheme is created or updated when updating an active workflow scheme. The draft is updated with the new default workflow. Defaults to `false`.
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow to set as the default workflow.
    pub workflow: String,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the mapping between an issue type and a workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeWorkflowMapping {
    /// The ID of the issue type. Not required if updating the issue type-workflow mapping.
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    /// Set to true to create or update the draft of a workflow scheme and update the mapping in the draft, when the workflow scheme cannot be edited. Defaults to `false`. Only applicable when updating the workflow-issue types mapping.
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

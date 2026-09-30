// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about the mapping between issue types and a workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypesWorkflowMapping {
    /// Whether the workflow is the default workflow for the workflow scheme.
    #[serde(rename = "defaultMapping", default, skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<bool>,
    /// The list of issue type IDs.
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    /// Whether a draft workflow scheme is created or updated when updating an active workflow scheme. The draft is updated with the new workflow-issue types mapping. Defaults to `false`.
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    /// The name of the workflow. Optional if updating the workflow-issue types mapping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

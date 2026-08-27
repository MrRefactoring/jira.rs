// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowMapping {
    #[serde(rename = "defaultMapping", default, skip_serializing_if = "Option::is_none")]
    pub default_mapping: Option<bool>,
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

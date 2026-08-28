// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeMapping {
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[serde(rename = "updateDraftIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub update_draft_if_needed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
}

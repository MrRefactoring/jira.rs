// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueRankRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    #[serde(rename = "rankAfterIssue", default, skip_serializing_if = "Option::is_none")]
    pub rank_after_issue: Option<String>,
    #[serde(rename = "rankBeforeIssue", default, skip_serializing_if = "Option::is_none")]
    pub rank_before_issue: Option<String>,
    #[serde(rename = "rankCustomFieldId", default, skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}

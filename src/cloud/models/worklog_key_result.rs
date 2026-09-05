// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorklogKeyResult {
    /// The issue ID.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    /// The worklog ID.
    #[serde(rename = "worklogId", default, skip_serializing_if = "Option::is_none")]
    pub worklog_id: Option<i64>,
}

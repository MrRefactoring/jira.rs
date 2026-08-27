// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Additional details about a project.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectInsight {
    /// The last issue update time.
    #[serde(
        rename = "lastIssueUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_issue_update_time: Option<String>,
    /// Total issue count.
    #[serde(rename = "totalIssueCount", default, skip_serializing_if = "Option::is_none")]
    pub total_issue_count: Option<i64>,
}

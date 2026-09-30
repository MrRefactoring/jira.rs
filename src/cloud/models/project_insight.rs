// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Additional details about a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectInsight {
    /// The last issue update time.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastIssueUpdateTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_issue_update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The last issue update time.
    #[cfg(not(feature = "chrono"))]
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

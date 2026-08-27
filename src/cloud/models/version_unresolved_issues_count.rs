// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Count of a version's unresolved issues.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionUnresolvedIssuesCount {
    /// Count of issues.
    #[serde(rename = "issuesCount", default, skip_serializing_if = "Option::is_none")]
    pub issues_count: Option<i64>,
    /// Count of unresolved issues.
    #[serde(rename = "issuesUnresolvedCount", default, skip_serializing_if = "Option::is_none")]
    pub issues_unresolved_count: Option<i64>,
    /// The URL of these count details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

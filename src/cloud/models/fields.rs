// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Key fields from the linked issue.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Fields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserDetails>,
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<IssueTypeDetails>,
    /// The type of the linked issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuetype: Option<IssueTypeDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusDetails>,
    /// The summary description of the linked issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timetracking: Option<TimeTrackingDetails>,
}

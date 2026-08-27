// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of changelogs that belong to single issue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueChangeLog {
    /// List of changelogs that belongs to given issueId.
    #[serde(rename = "changeHistories", default, skip_serializing_if = "Option::is_none")]
    pub change_histories: Option<Vec<Changelog>>,
    /// The ID of the issue.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
}

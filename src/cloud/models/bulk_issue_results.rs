// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of requested issues & fields.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkIssueResults {
    /// When Jira can't return an issue enumerated in a request due to a retriable error or payload constraint, we'll return the respective issue ID with a corresponding error message. This list is empty when there are no errors Issues which aren't found or that the user doesn't have permission to view won't be returned in this list.
    #[serde(rename = "issueErrors", default, skip_serializing_if = "Option::is_none")]
    pub issue_errors: Option<Vec<IssueError>>,
    /// The list of issues.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<Issue>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Describes the error that occurred when retrieving data for a particular issue.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueError {
    /// The error that occurred when fetching this issue.
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The ID of the issue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

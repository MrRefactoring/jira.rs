// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about the issues created and the errors for requests that failed.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatedIssues {
    /// Error details for failed issue creation requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BulkOperationErrorResult>>,
    /// Details of the issues created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<CreatedIssue>>,
}

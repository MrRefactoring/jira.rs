// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A list of the issues matched to a JQL query or details of errors encountered during matching.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueMatchesForJQL {
    /// A list of errors.
    pub errors: Vec<String>,
    /// A list of issue IDs.
    #[serde(rename = "matchedIssues")]
    pub matched_issues: Vec<i64>,
}

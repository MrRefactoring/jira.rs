// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Project and issue type context for workflow queries made using issue types.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectIssueTypeQueryContext {
    /// The set of issue type IDs.
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<String>>,
    /// The ID of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Status details for an issue type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeWithStatus {
    /// The ID of the issue type.
    pub id: String,
    /// The name of the issue type.
    pub name: String,
    /// The URL of the issue type's status details.
    #[serde(rename = "self")]
    pub self_: String,
    /// List of status details for the issue type.
    pub statuses: Vec<StatusDetails>,
    /// Whether this issue type represents subtasks.
    pub subtask: bool,
}

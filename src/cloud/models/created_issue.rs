// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a created issue or subtask.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatedIssue {
    /// The ID of the created issue or subtask.
    pub id: String,
    /// The key of the created issue or subtask.
    pub key: String,
    /// The URL of the created issue or subtask.
    #[serde(rename = "self")]
    pub self_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<NestedResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<NestedResponse>,
}

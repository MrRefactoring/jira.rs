// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeCreate {
    /// The description of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The hierarchy level of the issue type. Use:
    ///
    ///  *  `-1` for Subtask.
    ///  *  `0` for Base.
    ///
    /// Defaults to `0`.
    #[serde(rename = "hierarchyLevel", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i64>,
    /// The unique name for the issue type. The maximum length is 60 characters.
    pub name: String,
}

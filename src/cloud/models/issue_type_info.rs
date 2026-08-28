// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeInfo {
    /// The avatar of the issue type.
    #[serde(rename = "avatarId", default, skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The ID of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

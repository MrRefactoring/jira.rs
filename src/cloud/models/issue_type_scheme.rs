// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue type scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeScheme {
    /// The ID of the default issue type of the issue type scheme.
    #[serde(rename = "defaultIssueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue type scheme.
    pub id: String,
    /// Whether the issue type scheme is the default.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the issue type scheme.
    pub name: String,
}

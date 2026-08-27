// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue type scheme and its associated issue types.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemeDetails {
    /// The ID of the default issue type of the issue type scheme. This ID must be included in `issueTypeIds`.
    #[serde(rename = "defaultIssueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme. The maximum length is 4000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The list of issue types IDs of the issue type scheme. At least one standard issue type ID is required.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The name of the issue type scheme. The name must be unique. The maximum length is 255 characters.
    pub name: String,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the name, description, and default issue type for an issue type scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemeUpdateDetails {
    /// The ID of the default issue type of the issue type scheme.
    #[serde(rename = "defaultIssueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<String>,
    /// The description of the issue type scheme. The maximum length is 4000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue type scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

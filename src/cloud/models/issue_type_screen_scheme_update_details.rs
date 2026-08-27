// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue type screen scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeUpdateDetails {
    /// The description of the issue type screen scheme. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue type screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

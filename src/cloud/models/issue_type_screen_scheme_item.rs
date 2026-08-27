// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The screen scheme for an issue type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeItem {
    /// The ID of the issue type or *default*. Only issue types used in classic projects are accepted. When creating an issue screen scheme, an entry for *default* must be provided and defines the mapping for all issue types without a screen scheme. Otherwise, a *default* entry can't be provided.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the issue type screen scheme.
    #[serde(rename = "issueTypeScreenSchemeId")]
    pub issue_type_screen_scheme_id: String,
    /// The ID of the screen scheme.
    #[serde(rename = "screenSchemeId")]
    pub screen_scheme_id: String,
}

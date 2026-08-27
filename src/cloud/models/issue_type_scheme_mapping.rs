// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Issue type scheme item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemeMapping {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the issue type scheme.
    #[serde(rename = "issueTypeSchemeId")]
    pub issue_type_scheme_id: String,
}

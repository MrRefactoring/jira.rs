// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The ID of an issue type scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeSchemeID {
    /// The ID of the issue type scheme.
    #[serde(rename = "issueTypeSchemeId")]
    pub issue_type_scheme_id: String,
}

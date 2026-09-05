// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The list of issue types.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusProjectIssueTypeUsage {
    /// The issue type ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

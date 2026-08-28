// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The identifiers for a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectIdentifier {
    /// The ID of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

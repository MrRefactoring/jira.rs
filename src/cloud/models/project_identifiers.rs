// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Identifiers for a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectIdentifiers {
    /// The ID of the created project.
    pub id: i64,
    /// The key of the created project.
    pub key: String,
    /// The URL of the created project.
    #[serde(rename = "self")]
    pub self_: String,
}

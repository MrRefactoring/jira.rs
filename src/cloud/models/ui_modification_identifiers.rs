// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Identifiers for a UI modification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UiModificationIdentifiers {
    /// The ID of the UI modification.
    pub id: String,
    /// The URL of the UI modification.
    #[serde(rename = "self")]
    pub self_: String,
}

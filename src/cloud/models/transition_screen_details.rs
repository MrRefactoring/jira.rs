// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The details of a transition screen.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransitionScreenDetails {
    /// The ID of the screen.
    pub id: String,
    /// The name of the screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

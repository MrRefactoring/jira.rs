// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerTransition {
    /// ID of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

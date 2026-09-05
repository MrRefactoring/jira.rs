// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomerRequestAction {
    /// Indicates whether the user can undertake the action (true) or not (false).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
}

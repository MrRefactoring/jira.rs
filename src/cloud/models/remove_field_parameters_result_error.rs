// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Error during remove field parameters operation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoveFieldParametersResultError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

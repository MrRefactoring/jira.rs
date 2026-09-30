// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestFieldValidationError {
    /// The id of the request field that failed validation (matches a key in 'requestFieldValues').
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// A human-readable explanation of why this field failed validation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

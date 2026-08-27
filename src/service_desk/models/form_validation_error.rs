// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormValidationError {
    /// Machine-readable validation error code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Identifies the form entity that caused the validation error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<FormValidationErrorContext>>,
    /// Detailed validation error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// The HTTP status code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Short summary of the validation error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

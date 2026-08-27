// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Errors of bulk edit action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkEditActionError {
    /// The error messages.
    #[serde(rename = "errorMessages")]
    pub error_messages: Vec<String>,
    /// The errors.
    pub errors: std::collections::HashMap<String, serde_json::Value>,
}

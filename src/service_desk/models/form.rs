// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Form {
    /// JSON mapping of form field answers containing form field IDs and corresponding values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

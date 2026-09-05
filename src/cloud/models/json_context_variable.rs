// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Type of custom context variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JsonContextVariableType {
    #[serde(rename = "json")]
    Json,
}

/// A JSON object with custom content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonContextVariable {
    /// Type of custom context variable.
    pub r#type: JsonContextVariableType,
    /// A JSON object containing custom content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<std::collections::HashMap<String, serde_json::Value>>,
}

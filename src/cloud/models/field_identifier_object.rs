// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Identifier for a field for example FIELD\_ID.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldIdentifierObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub r#type: String,
}

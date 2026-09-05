// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Object schema and mapping configuration for an external import.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ImportSchemaAndMapping {
    /// Object schema and status schema configuration for the import.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Object type and attribute mapping configuration for the import.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<std::collections::HashMap<String, serde_json::Value>>,
}

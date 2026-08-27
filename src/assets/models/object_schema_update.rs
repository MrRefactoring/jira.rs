// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Used to update object schema
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectSchemaUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectSchemaKey", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

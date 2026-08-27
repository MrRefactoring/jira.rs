// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusIn {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// | Name | Value | Color |
    /// | ---- | ----- | ----- |
    /// | ACTIVE | 1 | Green |
    /// | INACTIVE | 0 | Red |
    /// | PENDING | 2 | Yellow |
    pub category: i64,
    #[serde(rename = "objectSchemaId", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_id: Option<String>,
}

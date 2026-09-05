// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// An Assets status type that can be associated with objects
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Status {
    pub id: String,
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

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Usage statistics for a single object schema within the tenant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaUsageInfo {
    /// The unique identifier of the schema.
    #[serde(rename = "schemaId")]
    pub schema_id: i64,
    /// The display name of the schema.
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    /// The timestamp when the schema was created (ISO 8601).
    #[serde(rename = "schemaCreatedAt", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub schema_created_at: String,
    /// The number of objects in this schema.
    #[serde(rename = "objectCount")]
    pub object_count: i64,
}

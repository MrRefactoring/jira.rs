// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Usage statistics for a single object schema within the tenant.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SchemaUsageInfo {
    /// The unique identifier of the schema.
    #[serde(rename = "schemaId")]
    pub schema_id: i64,
    /// The display name of the schema.
    #[serde(rename = "schemaName")]
    pub schema_name: String,
    /// The timestamp when the schema was created (ISO 8601).
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "schemaCreatedAt",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub schema_created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The timestamp when the schema was created (ISO 8601).
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "schemaCreatedAt", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub schema_created_at: String,
    /// The number of objects in this schema.
    #[serde(rename = "objectCount")]
    pub object_count: i64,
}

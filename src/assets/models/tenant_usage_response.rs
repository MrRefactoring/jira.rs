// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Comprehensive usage statistics for a tenant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantUsageResponse {
    /// Total number of objects across all schemas in the tenant.
    #[serde(rename = "totalObjectsCount")]
    pub total_objects_count: i64,
    /// Per-schema breakdown of usage information.
    #[serde(rename = "perSchemaUsageInfo")]
    pub per_schema_usage_info: Vec<SchemaUsageInfo>,
}

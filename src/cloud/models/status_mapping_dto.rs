// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The mapping of old to new status ID for a specific project and issue type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusMappingDTO {
    /// The issue type for the status mapping.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The project for the status mapping.
    #[serde(rename = "projectId")]
    pub project_id: String,
    /// The list of old and new status ID mappings for the specified project and issue type.
    #[serde(rename = "statusMigrations")]
    pub status_migrations: Vec<StatusMigration>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

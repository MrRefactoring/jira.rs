// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowSchemeMigrationRequest {
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<i64>,
    #[serde(rename = "statusMappings", default, skip_serializing_if = "Option::is_none")]
    pub status_mappings: Option<Vec<StatusMapping>>,
}

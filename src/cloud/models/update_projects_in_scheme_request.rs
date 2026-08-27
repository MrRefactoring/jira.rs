// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Update projects in a scheme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProjectsInSchemeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<PrioritySchemeChangesWithoutMappings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<PrioritySchemeChangesWithoutMappings>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

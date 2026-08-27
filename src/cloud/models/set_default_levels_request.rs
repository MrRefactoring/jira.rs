// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of new default levels.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDefaultLevelsRequest {
    /// List of objects with issue security scheme ID and new default level ID.
    #[serde(rename = "defaultValues")]
    pub default_values: Vec<DefaultLevelValue>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about an issue security scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecuritySchemeWithProjects {
    /// The default level ID of the issue security scheme.
    #[serde(rename = "defaultLevel", default, skip_serializing_if = "Option::is_none")]
    pub default_level: Option<i64>,
    /// The description of the issue security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue security scheme.
    pub id: i64,
    /// The name of the issue security scheme.
    pub name: String,
    /// The list of project IDs associated with the issue security scheme.
    #[serde(rename = "projectIds", default, skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    /// The URL of the issue security scheme.
    #[serde(rename = "self")]
    pub self_: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of an issue resolution.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateResolutionDetails {
    /// The description of the resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the resolution. Must be unique.
    pub name: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

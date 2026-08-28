// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of issue security scheme level.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateIssueSecurityLevelDetails {
    /// The description of the issue security scheme level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue security scheme level. Must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

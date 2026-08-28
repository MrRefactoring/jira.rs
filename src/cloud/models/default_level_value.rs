// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of scheme and new default level.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultLevelValue {
    /// The ID of the issue security level to set as default for the specified scheme. Providing null will reset the default level.
    #[serde(rename = "defaultLevelId")]
    pub default_level_id: String,
    /// The ID of the issue security scheme to set default level for.
    #[serde(rename = "issueSecuritySchemeId")]
    pub issue_security_scheme_id: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

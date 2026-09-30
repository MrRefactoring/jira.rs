// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a security scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityScheme {
    /// The ID of the default security level.
    #[serde(rename = "defaultSecurityLevelId", default, skip_serializing_if = "Option::is_none")]
    pub default_security_level_id: Option<i64>,
    /// The description of the issue security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SecurityLevel>>,
    /// The name of the issue security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the issue security scheme.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

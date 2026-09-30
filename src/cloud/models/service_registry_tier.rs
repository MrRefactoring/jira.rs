// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServiceRegistryTier {
    /// tier description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// tier ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// tier level
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// tier name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// name key of the tier
    #[serde(rename = "nameKey", default, skip_serializing_if = "Option::is_none")]
    pub name_key: Option<String>,
}

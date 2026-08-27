// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeBuildInfo {
    #[serde(rename = "buildNumber", default, skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

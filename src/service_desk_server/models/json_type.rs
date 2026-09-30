// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JsonType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[serde(rename = "customId", default, skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

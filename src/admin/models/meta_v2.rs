// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MetaV2 {
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "startIndex", default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(rename = "endIndex", default, skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
}

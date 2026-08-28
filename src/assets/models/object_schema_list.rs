// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectSchemaList {
    #[serde(rename = "startAt")]
    pub start_at: i64,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    pub total: i64,
    pub values: Vec<ObjectSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<bool>,
    #[serde(rename = "isLast", default, skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
}

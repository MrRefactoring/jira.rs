// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderByOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<OrderByOption>>,
    #[serde(rename = "matchesCount", default, skip_serializing_if = "Option::is_none")]
    pub matches_count: Option<i64>,
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

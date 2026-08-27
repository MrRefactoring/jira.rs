// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page<T> {
    #[serde(rename = "isLast")]
    pub is_last: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    #[serde(rename = "startAt")]
    pub start_at: i64,
    pub total: i64,
    pub values: Vec<T>,
}

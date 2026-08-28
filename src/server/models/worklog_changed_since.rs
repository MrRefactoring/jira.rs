// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorklogChangedSince {
    #[serde(rename = "isLastPage", default, skip_serializing_if = "Option::is_none")]
    pub is_last_page: Option<bool>,
    #[serde(rename = "lastPage", default, skip_serializing_if = "Option::is_none")]
    pub last_page: Option<bool>,
    #[serde(rename = "nextPage", default, skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<WorklogChange>>,
}

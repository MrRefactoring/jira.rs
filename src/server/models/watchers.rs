// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Watchers {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "isWatching", default, skip_serializing_if = "Option::is_none")]
    pub is_watching: Option<bool>,
    #[serde(rename = "watchCount", default, skip_serializing_if = "Option::is_none")]
    pub watch_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Vec<UserJson>>,
}

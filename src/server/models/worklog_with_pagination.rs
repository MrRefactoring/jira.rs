// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorklogWithPagination {
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "startAt", default, skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worklogs: Option<Vec<Worklog>>,
}

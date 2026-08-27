// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionUnresolvedIssueCounts {
    #[serde(rename = "issuesUnresolvedCount", default, skip_serializing_if = "Option::is_none")]
    pub issues_unresolved_count: Option<i64>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ComponentIssueCounts {
    #[serde(rename = "issueCount", default, skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

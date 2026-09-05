// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorklogsMoveRequest {
    /// A list of worklog IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    /// The issue id or key of the destination issue
    #[serde(rename = "issueIdOrKey", default, skip_serializing_if = "Option::is_none")]
    pub issue_id_or_key: Option<String>,
}

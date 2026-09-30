// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTransitionStatus {
    /// The unique ID of the status.
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i64>,
    /// The name of the status.
    #[serde(rename = "statusName", default, skip_serializing_if = "Option::is_none")]
    pub status_name: Option<String>,
}

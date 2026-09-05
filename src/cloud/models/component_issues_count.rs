// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Count of issues assigned to a component.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ComponentIssuesCount {
    /// The count of issues assigned to a component.
    #[serde(rename = "issueCount", default, skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i64>,
    /// The URL for this count of issues for a component.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a link between issues.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IssueLink {
    /// The ID of the issue link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: LinkedIssue,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: LinkedIssue,
    /// The URL of the issue link.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    pub r#type: IssueLinkType,
}

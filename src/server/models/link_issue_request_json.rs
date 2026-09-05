// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkIssueRequestJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentJson>,
    #[serde(rename = "inwardIssue", default, skip_serializing_if = "Option::is_none")]
    pub inward_issue: Option<IssueRefJson>,
    #[serde(rename = "outwardIssue", default, skip_serializing_if = "Option::is_none")]
    pub outward_issue: Option<IssueRefJson>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IssueLinkTypeJson>,
}

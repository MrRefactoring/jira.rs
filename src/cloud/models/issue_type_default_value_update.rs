// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A default value update for one issue-type scope in a context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeDefaultValueUpdate {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: i64,
    /// True when this is the catch-all default for issue types without a specific default.
    #[serde(rename = "isAnyIssueType", default, skip_serializing_if = "Option::is_none")]
    pub is_any_issue_type: Option<bool>,
    /// The ID of the issue type this default value applies to.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<CustomFieldContextDefaultValue>,
}

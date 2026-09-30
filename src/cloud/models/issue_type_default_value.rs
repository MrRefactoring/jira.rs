// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A default value associated with an issue type within a context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeDefaultValue {
    /// True when this default value applies to every issue type covered by the context (no specific issue type). Only present when true; omitted otherwise.
    #[serde(rename = "isAnyIssueType", default, skip_serializing_if = "Option::is_none")]
    pub is_any_issue_type: Option<bool>,
    /// The ID of the issue type this default value applies to. Null when isAnyIssueType is true.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<CustomFieldContextDefaultValue>,
}

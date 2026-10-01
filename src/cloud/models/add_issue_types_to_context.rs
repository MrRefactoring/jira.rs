// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddIssueTypesToContext {
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<CustomFieldContextDefaultValue>,
    /// Whether to add or retain the any-issue-type mapping. At least one of this property or issueTypeIds is required. Defaults to false.
    #[serde(rename = "isAnyIssueType", default, skip_serializing_if = "Option::is_none")]
    pub is_any_issue_type: Option<bool>,
    /// The issue type IDs to add. Optional when isAnyIssueType is true.
    #[serde(rename = "issueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<Option<String>>>,
}

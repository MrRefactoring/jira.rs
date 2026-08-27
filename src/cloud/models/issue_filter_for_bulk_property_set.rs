// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Bulk operation filter details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFilterForBulkPropertySet {
    /// The value of properties to perform the bulk operation on.
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<serde_json::Value>,
    /// List of issues to perform the bulk operation on.
    #[serde(rename = "entityIds", default, skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<i64>>,
    /// Whether the bulk operation occurs only when the property is present on or absent from an issue.
    #[serde(rename = "hasProperty", default, skip_serializing_if = "Option::is_none")]
    pub has_property: Option<bool>,
}

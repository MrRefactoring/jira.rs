// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SimplifiedHierarchyLevel {
    #[serde(rename = "hierarchyLevelNumber", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_level_number: Option<i64>,
    /// The issue types available in this hierarchy level.
    #[serde(rename = "issueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<i64>>,
    /// The level of this item in the hierarchy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// The name of this hierarchy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

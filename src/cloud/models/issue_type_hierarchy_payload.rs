// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The conflict strategy to use when the issue type already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters
    pub enum IssueTypeHierarchyPayloadOnConflict {
        Fail => "FAIL",
        Use => "USE",
        New => "NEW",
    }
}

/// The payload for creating an issue type hierarchy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeHierarchyPayload {
    /// The hierarchy level of the issue type. 0, 1, 2, 3 .. n; Negative values for subtasks
    #[serde(rename = "hierarchyLevel", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i64>,
    /// The name of the issue type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The conflict strategy to use when the issue type already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters
    #[serde(rename = "onConflict", default, skip_serializing_if = "Option::is_none")]
    pub on_conflict: Option<IssueTypeHierarchyPayloadOnConflict>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of an issue type hierarchy level.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectIssueTypesHierarchyLevel {
    /// The list of issue types in the hierarchy level.
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<IssueTypeInfo>>,
    /// The level of the issue type hierarchy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// The name of the issue type hierarchy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

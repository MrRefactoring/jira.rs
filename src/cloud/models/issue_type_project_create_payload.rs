// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for creating issue types in a project
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeProjectCreatePayload {
    /// Defines the issue type hierarhy to be created and used during this project creation. This will only add new levels if there isn't an existing level
    #[serde(rename = "issueTypeHierarchy", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_hierarchy: Option<Vec<IssueTypeHierarchyPayload>>,
    #[serde(rename = "issueTypeScheme", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_scheme: Option<IssueTypeSchemePayload>,
    /// Only needed if you want to create issue types, you can otherwise use the ids of issue types in the scheme configuration
    #[serde(rename = "issueTypes", default, skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Vec<IssueTypePayload>>,
}

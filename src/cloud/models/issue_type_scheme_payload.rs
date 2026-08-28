// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The payload for creating issue type schemes
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemePayload {
    #[serde(rename = "defaultIssueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub default_issue_type_id: Option<ProjectCreateResourceIdentifier>,
    /// The description of the issue type scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The issue type IDs for the issue type scheme
    #[serde(rename = "issueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<ProjectCreateResourceIdentifier>>,
    /// The name of the issue type scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
}

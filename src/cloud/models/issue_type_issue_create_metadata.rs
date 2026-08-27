// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the issue creation metadata for an issue type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeIssueCreateMetadata {
    /// The ID of the issue type's avatar.
    #[serde(rename = "avatarId", default, skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique ID for next-gen projects.
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Expand options that include additional issue type metadata details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of the fields available when creating an issue for the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Hierarchy level of the issue type.
    #[serde(rename = "hierarchyLevel", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i64>,
    /// The URL of the issue type's avatar.
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the issue type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The URL of these issue type details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Whether this issue type is used to create subtasks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}

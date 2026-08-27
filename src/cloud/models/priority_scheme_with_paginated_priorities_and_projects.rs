// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A priority scheme with paginated priorities and projects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrioritySchemeWithPaginatedPrioritiesAndProjects {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The ID of the default issue priority.
    #[serde(rename = "defaultPriorityId", default, skip_serializing_if = "Option::is_none")]
    pub default_priority_id: Option<String>,
    /// The description of the priority scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the priority scheme.
    pub id: String,
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The name of the priority scheme
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priorities: Option<PagePriorityWithSequence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<PageProjectDetails>,
    /// The URL of the priority scheme.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

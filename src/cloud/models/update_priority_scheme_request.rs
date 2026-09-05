// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a priority scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdatePrioritySchemeRequest {
    /// The default priority of the scheme.
    #[serde(rename = "defaultPriorityId", default, skip_serializing_if = "Option::is_none")]
    pub default_priority_id: Option<i64>,
    /// The description of the priority scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<PriorityMapping>,
    /// The name of the priority scheme. Must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priorities: Option<UpdatePrioritiesInSchemeRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<UpdateProjectsInSchemeRequest>,
}

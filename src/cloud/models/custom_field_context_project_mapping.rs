// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a context to project association.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextProjectMapping {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// Whether context is global.
    #[serde(rename = "isGlobalContext", default, skip_serializing_if = "Option::is_none")]
    pub is_global_context: Option<bool>,
    /// The ID of the project.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

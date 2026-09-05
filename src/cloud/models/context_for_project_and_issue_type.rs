// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The project and issue type mapping with a matching custom field context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContextForProjectAndIssueType {
    /// The ID of the custom field context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

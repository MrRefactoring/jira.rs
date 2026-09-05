// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueProjectType {
    #[serde(rename = "project")]
    Project,
}

/// The default value for a project custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueProject {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The ID of the default project.
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub r#type: CustomFieldContextDefaultValueProjectType,
}

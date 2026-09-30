// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Associated field configuration scheme and project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConfigurationSchemeProjectAssociation {
    /// The ID of the field configuration scheme. If the field configuration scheme ID is `null`, the operation assigns the default field configuration scheme.
    #[serde(rename = "fieldConfigurationSchemeId", default, skip_serializing_if = "Option::is_none")]
    pub field_configuration_scheme_id: Option<String>,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

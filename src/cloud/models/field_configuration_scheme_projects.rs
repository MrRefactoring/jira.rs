// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Project list with assigned field configuration schema.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfigurationSchemeProjects {
    #[serde(rename = "fieldConfigurationScheme", default, skip_serializing_if = "Option::is_none")]
    pub field_configuration_scheme: Option<FieldConfigurationScheme>,
    /// The IDs of projects using the field configuration scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

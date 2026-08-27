// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Request to create a project using a custom template
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectCustomTemplateCreateRequestDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<CustomTemplatesProjectDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<CustomTemplateRequestDTO>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForgePanelProjectPinRequest {
    /// The moduleId of the Forge panel in the format `ari:cloud:ecosystem::extension/{app-id}/{environment-id}/static/{module-key}`
    #[serde(rename = "moduleId")]
    pub module_id: String,
    /// The list of projects to pin or unpin the issue panel to or from.
    #[serde(rename = "projectList")]
    pub project_list: Vec<ProjectPinAction>,
}

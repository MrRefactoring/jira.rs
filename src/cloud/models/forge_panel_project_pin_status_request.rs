// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForgePanelProjectPinStatusRequest {
    /// The moduleId of the Forge panel in the format `ari:cloud:ecosystem::extension/{app-id}/{environment-id}/static/{module-key}`
    #[serde(rename = "moduleId")]
    pub module_id: String,
    /// The IDs or keys of the projects to check the issue panel pin status for.
    #[serde(rename = "projectList")]
    pub project_list: Vec<String>,
}

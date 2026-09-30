// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForgePanelProjectPinStatusResponse {
    /// The moduleId of the Forge panel that was requested.
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    /// The pin status of the issue panel, with one entry per requested project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ForgePanelProjectPinStatus>>,
}

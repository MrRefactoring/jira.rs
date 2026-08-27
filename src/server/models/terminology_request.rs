// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerminologyRequest {
    #[serde(rename = "newName", default, skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "newNamePlural", default, skip_serializing_if = "Option::is_none")]
    pub new_name_plural: Option<String>,
    #[serde(rename = "originalName", default, skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
}

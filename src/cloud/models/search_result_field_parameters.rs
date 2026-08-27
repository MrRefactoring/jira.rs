// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchResultFieldParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "rendererType", default, skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
}

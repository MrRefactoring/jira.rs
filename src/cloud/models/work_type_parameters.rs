// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkTypeParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "rendererType", default, skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
    #[serde(rename = "workTypeId")]
    pub work_type_id: i64,
}

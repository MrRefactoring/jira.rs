// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response object for getting field association parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFieldAssociationParametersResponse {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FieldAssociationParameters>,
    #[serde(rename = "workTypeParameters", default, skip_serializing_if = "Option::is_none")]
    pub work_type_parameters: Option<Vec<WorkTypeParameters>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Partial failure result when updating field scheme to fields associations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldSchemeToFieldsPartialFailure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "schemeId")]
    pub scheme_id: i64,
    pub success: bool,
    #[serde(rename = "workTypeIds")]
    pub work_type_ids: Vec<i64>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParameterRemovalDetails {
    /// Set of parameter names to remove
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    /// ID of the field scheme
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<i64>,
    /// Set of work type (issue type) IDs
    #[serde(rename = "workTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub work_type_ids: Option<Vec<i64>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProjectField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "workTypeId", default, skip_serializing_if = "Option::is_none")]
    pub work_type_id: Option<i64>,
}

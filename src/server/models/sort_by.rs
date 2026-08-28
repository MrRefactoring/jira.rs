// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SortBy {
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(rename = "toggleJql", default, skip_serializing_if = "Option::is_none")]
    pub toggle_jql: Option<String>,
}

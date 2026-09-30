// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderByOption {
    #[serde(rename = "fieldId", default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "sortJql", default, skip_serializing_if = "Option::is_none")]
    pub sort_jql: Option<String>,
}

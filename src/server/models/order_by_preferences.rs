// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderByPreferences {
    #[serde(rename = "orderByOption", default, skip_serializing_if = "Option::is_none")]
    pub order_by_option: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}

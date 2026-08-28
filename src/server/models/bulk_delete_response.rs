// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkDeleteResponse {
    #[serde(rename = "deletedCustomFields", default, skip_serializing_if = "Option::is_none")]
    pub deleted_custom_fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "notDeletedCustomFields", default, skip_serializing_if = "Option::is_none")]
    pub not_deleted_custom_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

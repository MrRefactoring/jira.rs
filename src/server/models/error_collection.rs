// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ErrorCollection {
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, serde_json::Value>>,
}

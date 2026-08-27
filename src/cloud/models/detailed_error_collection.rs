// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetailedErrorCollection {
    /// Map of objects representing additional details for an error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The list of error messages produced by this operation. For example, "input parameter 'key' must be provided"
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// The list of errors by parameter returned by the operation. For example,"projectKey": "Project keys must start with an uppercase letter, followed by one or more uppercase alphanumeric characters."
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, serde_json::Value>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequestTypeFieldValue {
    /// List of child fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<RequestTypeFieldValue>>,
    /// Label for the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Value of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueReadOnlyType {
    #[serde(rename = "readonly")]
    Readonly,
}

/// The default text for a read only custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueReadOnly {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: CustomFieldContextDefaultValueReadOnlyType,
}

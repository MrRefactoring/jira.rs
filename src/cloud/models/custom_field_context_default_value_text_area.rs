// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueTextAreaType {
    #[serde(rename = "textarea")]
    Textarea,
}

/// The default text for a text area custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueTextArea {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. The maximum length is 32767 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: CustomFieldContextDefaultValueTextAreaType,
}

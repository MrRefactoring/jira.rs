// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueTextFieldType {
    #[serde(rename = "textfield")]
    Textfield,
}

/// The default text for a text custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueTextField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. The maximum length is 254 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: CustomFieldContextDefaultValueTextFieldType,
}

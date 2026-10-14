// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueLongTextType {
    #[serde(rename = "longtext")]
    Longtext,
}

/// The default text for a plain text paragraph custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueLongText {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. Line breaks are preserved. The maximum length is the instance's text field character limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: CustomFieldContextDefaultValueLongTextType,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeStringFieldType {
    #[serde(rename = "forge.string")]
    ForgeString,
}

/// The default text for a Forge string custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueForgeStringField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default text. The maximum length is 254 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub r#type: CustomFieldContextDefaultValueForgeStringFieldType,
}

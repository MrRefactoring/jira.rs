// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeMultiStringFieldType {
    #[serde(rename = "forge.string.list")]
    ForgeStringList,
}

/// The default text for a Forge collection of strings custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiStringField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    pub r#type: CustomFieldContextDefaultValueForgeMultiStringFieldType,
    /// List of string values. The maximum length for a value is 254 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

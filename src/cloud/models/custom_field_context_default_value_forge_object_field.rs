// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeObjectFieldType {
    #[serde(rename = "forge.object")]
    ForgeObject,
}

/// The default value for a Forge object custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueForgeObjectField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default JSON object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub r#type: CustomFieldContextDefaultValueForgeObjectFieldType,
}

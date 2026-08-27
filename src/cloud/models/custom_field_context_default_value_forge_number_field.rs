// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeNumberFieldType {
    #[serde(rename = "forge.number")]
    ForgeNumber,
}

/// Default value for a Forge number custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeNumberField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default floating-point number.
    pub number: f64,
    pub r#type: CustomFieldContextDefaultValueForgeNumberFieldType,
}

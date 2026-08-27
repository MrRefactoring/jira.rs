// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueFloatType {
    #[serde(rename = "float")]
    Float,
}

/// Default value for a float (number) custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueFloat {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default floating-point number.
    pub number: f64,
    pub r#type: CustomFieldContextDefaultValueFloatType,
}

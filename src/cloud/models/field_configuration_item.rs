// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A field within a field configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfigurationItem {
    /// The description of the field within the field configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the field within the field configuration.
    pub id: String,
    /// Whether the field is hidden in the field configuration.
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    /// Whether the field is required in the field configuration.
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// The renderer type for the field within the field configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
}

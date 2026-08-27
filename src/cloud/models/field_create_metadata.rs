// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The metadata describing an issue field for createmeta.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldCreateMetadata {
    /// The list of values allowed in the field.
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<serde_json::Value>>,
    /// The URL that can be used to automatically complete the field.
    #[serde(rename = "autoCompleteUrl", default, skip_serializing_if = "Option::is_none")]
    pub auto_complete_url: Option<String>,
    /// The configuration properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The default value of the field.
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    /// The field id.
    #[serde(rename = "fieldId")]
    pub field_id: String,
    /// Whether the field has a default value.
    #[serde(rename = "hasDefaultValue", default, skip_serializing_if = "Option::is_none")]
    pub has_default_value: Option<bool>,
    /// The key of the field.
    pub key: String,
    /// The name of the field.
    pub name: String,
    /// The list of operations that can be performed on the field.
    pub operations: Vec<String>,
    /// Whether the field is required.
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonType>,
}

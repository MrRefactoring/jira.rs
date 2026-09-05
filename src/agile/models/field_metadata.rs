// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The schema of a field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldMetadataSchema {
    /// If the field is a custom field, the configuration of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// If the field is a custom field, the URI of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    /// If the field is a custom field, the custom ID of the field.
    #[serde(rename = "customId", default, skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<i64>,
    /// When the data type is an array, the name of the field items within the array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    /// If the field is a system field, the name of the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// The data type of the field.
    pub r#type: String,
}

/// The metadata describing an issue field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldMetadata {
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
    /// The schema of a field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<FieldMetadataSchema>,
}

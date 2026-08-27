// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueForgeDateTimeFieldType {
    #[serde(rename = "forge.datetime")]
    ForgeDatetime,
}

/// The default value for a Forge date time custom field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeDateTimeField {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default date-time in ISO format. Ignored if `useCurrent` is true.
    #[serde(rename = "dateTime", default, skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    pub r#type: CustomFieldContextDefaultValueForgeDateTimeFieldType,
    /// Whether to use the current date.
    #[serde(rename = "useCurrent", default, skip_serializing_if = "Option::is_none")]
    pub use_current: Option<bool>,
}

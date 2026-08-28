// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomFieldContextDefaultValueDateType {
    #[serde(rename = "datepicker")]
    Datepicker,
}

/// The default value for a Date custom field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldContextDefaultValueDate {
    /// The ID of the context.
    #[serde(rename = "contextId")]
    pub context_id: String,
    /// The default date in ISO format. Ignored if `useCurrent` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    pub r#type: CustomFieldContextDefaultValueDateType,
    /// Whether to use the current date.
    #[serde(rename = "useCurrent", default, skip_serializing_if = "Option::is_none")]
    pub use_current: Option<bool>,
}

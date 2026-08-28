// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a custom field option for a context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldOptionUpdate {
    /// Whether the option is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The ID of the custom field option.
    pub id: String,
    /// The value of the custom field option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

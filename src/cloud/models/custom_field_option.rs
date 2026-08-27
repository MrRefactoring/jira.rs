// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a custom option for a field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldOption {
    /// The URL of these custom field option details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The value of the custom field option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

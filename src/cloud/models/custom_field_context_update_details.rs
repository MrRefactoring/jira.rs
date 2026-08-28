// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a custom field context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextUpdateDetails {
    /// The description of the custom field context. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the custom field context. The name must be unique. The maximum length is 255 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

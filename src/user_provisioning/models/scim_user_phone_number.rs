// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// SCIM user phone number
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScimUserPhoneNumber {
    /// Phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Type of phone number, for example `work` or `personal`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Boolean value indicating whether phone number is primary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

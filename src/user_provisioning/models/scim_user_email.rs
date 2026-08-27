// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// SCIM user email
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScimUserEmail {
    /// Email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Type of email address, for example "work" or "personal".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Boolean value indicating whether this is the primary email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

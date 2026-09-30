// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAvatarUrls {
    /// The URL of the user's 16x16 pixel avatar.
    #[serde(rename = "16x16", default, skip_serializing_if = "Option::is_none")]
    pub n16x16: Option<String>,
    /// The URL of the user's 24x24 pixel avatar.
    #[serde(rename = "24x24", default, skip_serializing_if = "Option::is_none")]
    pub n24x24: Option<String>,
    /// The URL of the user's 32x32 pixel avatar.
    #[serde(rename = "32x32", default, skip_serializing_if = "Option::is_none")]
    pub n32x32: Option<String>,
    /// The URL of the user's 48x48 pixel avatar.
    #[serde(rename = "48x48", default, skip_serializing_if = "Option::is_none")]
    pub n48x48: Option<String>,
}

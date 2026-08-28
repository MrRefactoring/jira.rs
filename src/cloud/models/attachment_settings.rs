// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of the instance's attachment settings.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachmentSettings {
    /// Whether the ability to add attachments is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The maximum size of attachments permitted, in bytes.
    #[serde(rename = "uploadLimit", default, skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
}

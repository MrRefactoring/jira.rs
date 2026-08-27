// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporaryWebAttachment {
    #[serde(rename = "temporaryAttachmentId", default, skip_serializing_if = "Option::is_none")]
    pub temporary_attachment_id: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

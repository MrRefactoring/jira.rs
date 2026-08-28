// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachTemporaryFileTemporaryAttachments {
    /// The id to hand to `createAttachment` when attaching this file to a request.
    #[serde(rename = "temporaryAttachmentId", default, skip_serializing_if = "Option::is_none")]
    pub temporary_attachment_id: Option<String>,
    /// The name the file was uploaded under.
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachTemporaryFile {
    #[serde(rename = "temporaryAttachments", default, skip_serializing_if = "Option::is_none")]
    pub temporary_attachments: Option<Vec<AttachTemporaryFileTemporaryAttachments>>,
}

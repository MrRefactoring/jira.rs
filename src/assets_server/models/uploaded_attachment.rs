// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// When the attachment was stored, as whole seconds since the epoch and the nanoseconds after them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadedAttachmentCreated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nanos: Option<i64>,
}

/// An attachment as the upload reports it, whose `created` is a timestamp rather than a date.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadedAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesize: Option<String>,
    /// When the attachment was stored, as whole seconds since the epoch and the nanoseconds after them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<UploadedAttachmentCreated>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "commentOutput", default, skip_serializing_if = "Option::is_none")]
    pub comment_output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

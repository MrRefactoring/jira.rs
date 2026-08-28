// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about an attachment.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserDetails>,
    /// The content of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The datetime the attachment was created.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The datetime the attachment was created.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// The file name of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The ID of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The MIME type of the attachment.
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The URL of the attachment details response.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The size of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The URL of a thumbnail representing the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

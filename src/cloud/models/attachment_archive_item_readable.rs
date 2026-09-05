// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Metadata for an item in an attachment archive.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachmentArchiveItemReadable {
    /// The position of the item within the archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// The label for the archive item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The MIME type of the archive item.
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The path of the archive item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The size of the archive item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

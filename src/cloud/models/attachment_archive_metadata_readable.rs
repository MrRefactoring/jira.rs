// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Metadata for an archive (for example a zip) and its contents.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentArchiveMetadataReadable {
    /// The list of the items included in the archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AttachmentArchiveItemReadable>>,
    /// The ID of the attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The MIME type of the attachment.
    #[serde(rename = "mediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The name of the archive file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of items included in the archive.
    #[serde(rename = "totalEntryCount", default, skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
}

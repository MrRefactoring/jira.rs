// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachmentArchiveImpl {
    /// The list of the items included in the archive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AttachmentArchiveEntry>>,
    /// The number of items in the archive.
    #[serde(rename = "totalEntryCount", default, skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
}

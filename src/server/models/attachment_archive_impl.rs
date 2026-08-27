// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentArchiveImpl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AttachmentArchiveEntry>>,
    /// Total number of entries available (can be larger that what was asked for)
    #[serde(rename = "totalEntryCount", default, skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
}

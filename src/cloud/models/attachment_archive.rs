// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentArchive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AttachmentArchiveEntry>>,
    #[serde(rename = "moreAvailable", default, skip_serializing_if = "Option::is_none")]
    pub more_available: Option<bool>,
    #[serde(rename = "totalEntryCount", default, skip_serializing_if = "Option::is_none")]
    pub total_entry_count: Option<i64>,
    #[serde(rename = "totalNumberOfEntriesAvailable", default, skip_serializing_if = "Option::is_none")]
    pub total_number_of_entries_available: Option<i64>,
}

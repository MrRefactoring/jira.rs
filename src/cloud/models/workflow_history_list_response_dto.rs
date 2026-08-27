// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of workflow history entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowHistoryListResponseDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<WorkflowHistoryItemDTO>>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The specified workflow version read from history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowHistoryReadResponseDTO {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowDocumentStatusDTO>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowDocumentDTO>>,
}

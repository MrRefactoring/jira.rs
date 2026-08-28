// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The update workflows payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowUpdateRequest {
    /// The statuses to associate with the workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowStatusUpdate>>,
    /// The details of the workflows to update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowUpdate>>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The create workflows payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<WorkflowScope>,
    /// The statuses to associate with the workflows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<WorkflowStatusUpdate>>,
    /// The details of the workflows to create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Vec<WorkflowCreate>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The layout of the workflow status.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowStatusLayoutPayload {
    /// The x coordinate of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y coordinate of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

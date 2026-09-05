// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The starting point for the statuses in the workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowLayout {
    /// The x axis location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y axis location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

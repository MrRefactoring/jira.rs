// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Layout coordinates for workflow elements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowPreviewLayout {
    /// The X coordinate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The Y coordinate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

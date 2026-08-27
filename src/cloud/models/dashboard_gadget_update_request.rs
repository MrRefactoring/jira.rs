// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The details of the gadget to update.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardGadgetUpdateRequest {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<DashboardGadgetPosition>,
    /// The title of the gadget.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

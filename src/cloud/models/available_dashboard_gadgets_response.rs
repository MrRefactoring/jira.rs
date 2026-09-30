// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of available gadgets.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableDashboardGadgetsResponse {
    /// The list of available gadgets.
    pub gadgets: Vec<AvailableDashboardGadget>,
}

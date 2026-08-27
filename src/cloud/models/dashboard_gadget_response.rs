// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of gadgets on the dashboard.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardGadgetResponse {
    /// The list of gadgets.
    pub gadgets: Vec<DashboardGadget>,
}

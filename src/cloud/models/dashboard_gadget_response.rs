// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of gadgets on the dashboard.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DashboardGadgetResponse {
    /// The list of gadgets.
    pub gadgets: Vec<DashboardGadget>,
}

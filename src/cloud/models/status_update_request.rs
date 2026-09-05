// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The list of statuses that will be updated.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusUpdateRequest {
    /// The list of statuses that will be updated.
    pub statuses: Vec<StatusUpdate>,
}

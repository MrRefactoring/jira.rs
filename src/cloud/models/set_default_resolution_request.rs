// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The new default issue resolution.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetDefaultResolutionRequest {
    /// The ID of the new default issue resolution. Must be an existing ID or null. Setting this to null erases the default resolution setting.
    pub id: String,
}

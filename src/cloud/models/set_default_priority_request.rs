// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The new default issue priority.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultPriorityRequest {
    /// The ID of the new default issue priority. Must be an existing ID or null. Setting this to null erases the default priority setting.
    pub id: String,
}

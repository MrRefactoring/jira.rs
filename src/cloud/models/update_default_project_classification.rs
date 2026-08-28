// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The request for updating the default project classification level.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateDefaultProjectClassification {
    /// The ID of the project classification.
    pub id: String,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrioritySchemeChangesWithoutMappings {
    /// Affected entity ids.
    pub ids: Vec<i64>,
}

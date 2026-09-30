// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The ID and the name of the workflow scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowSchemeIdName {
    /// The ID of the workflow scheme.
    pub id: String,
    /// The name of the workflow scheme.
    pub name: String,
}

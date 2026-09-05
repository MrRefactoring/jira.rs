// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Project ID details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowProjectIdScope {
    /// The ID of the project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

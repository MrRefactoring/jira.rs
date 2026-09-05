// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Request for associating field schemes to projects.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldSchemeToProjectsRequest {
    /// List of project IDs to associate with field schemes
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<i64>,
}

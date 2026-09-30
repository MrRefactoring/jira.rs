// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A workflow scheme along with a list of projects that use it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowSchemeAssociations {
    /// The list of projects that use the workflow scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
    #[serde(rename = "workflowScheme")]
    pub workflow_scheme: WorkflowScheme,
}

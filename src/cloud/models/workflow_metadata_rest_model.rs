// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Workflow metadata and usage detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WorkflowMetadataRestModel {
    /// The description of the workflow.
    pub description: String,
    /// The ID of the workflow.
    pub id: String,
    /// The name of the workflow.
    pub name: String,
    pub version: DocumentVersion,
}

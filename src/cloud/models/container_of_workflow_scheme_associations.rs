// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A container for a list of workflow schemes together with the projects they are associated with.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContainerOfWorkflowSchemeAssociations {
    /// A list of workflow schemes together with projects they are associated with.
    pub values: Vec<WorkflowSchemeAssociations>,
}

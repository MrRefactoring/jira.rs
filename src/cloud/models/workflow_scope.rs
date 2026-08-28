// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The scope of the workflow. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
    pub enum WorkflowScopeType {
        Project => "PROJECT",
        Global => "GLOBAL",
    }
}

/// The scope of the workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectId>,
    /// The scope of the workflow. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WorkflowScopeType>,
}

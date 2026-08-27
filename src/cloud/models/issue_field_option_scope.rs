// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFieldOptionScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global: Option<GlobalScope>,
    /// DEPRECATED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
    /// Defines the projects in which the option is available and the behavior of the option within each project. Specify one object per project. The behavior of the option in a project context overrides the behavior in the global context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects2: Option<Vec<ProjectScope>>,
}

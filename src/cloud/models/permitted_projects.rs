// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of projects in which a user is granted permissions.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PermittedProjects {
    /// A list of projects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectIdentifier>>,
}

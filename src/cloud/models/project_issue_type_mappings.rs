// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The project and issue type mappings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectIssueTypeMappings {
    /// The project and issue type mappings.
    pub mappings: Vec<ProjectIssueTypeMapping>,
}

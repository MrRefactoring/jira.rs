// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue type scheme with a list of the projects that use it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeSchemeProjects {
    #[serde(rename = "issueTypeScheme")]
    pub issue_type_scheme: IssueTypeScheme,
    /// The IDs of the projects using the issue type scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

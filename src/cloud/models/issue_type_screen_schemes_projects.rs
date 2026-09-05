// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue type screen scheme with a list of the projects that use it.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueTypeScreenSchemesProjects {
    #[serde(rename = "issueTypeScreenScheme", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_screen_scheme: Option<IssueTypeScreenScheme>,
    /// The IDs of the projects using the issue type screen scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

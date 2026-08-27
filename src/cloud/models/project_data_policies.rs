// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about data policies for a list of projects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectDataPolicies {
    /// List of projects with data policies.
    #[serde(rename = "projectDataPolicies", default, skip_serializing_if = "Option::is_none")]
    pub project_data_policies: Option<Vec<ProjectWithDataPolicy>>,
}

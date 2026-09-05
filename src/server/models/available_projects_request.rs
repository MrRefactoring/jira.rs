// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableProjectsRequest {
    #[serde(rename = "ignoredProjectIds", default, skip_serializing_if = "Option::is_none")]
    pub ignored_project_ids: Option<Vec<String>>,
}

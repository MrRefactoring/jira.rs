// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Response item returned from get projects with field schemes.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetProjectsWithFieldSchemesResponse {
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "schemeId", default, skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<i64>,
}

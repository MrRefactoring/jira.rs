// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Matched filters for field association scheme search.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldAssociationSchemeMatchedFilters {
    #[serde(rename = "projectIds", default, skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

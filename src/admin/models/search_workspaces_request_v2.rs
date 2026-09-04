// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Workspaces request supporting enhanced Workspace searching.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchWorkspacesRequestV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<QueryVariants>,
    /// Specifies the maximum page size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortField>>,
    /// A base-64 encoded continuation token used for pagination. When a cursor is provided in the request body, no other properties may be present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

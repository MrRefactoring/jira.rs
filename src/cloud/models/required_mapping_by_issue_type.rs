// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The list of required status mappings by issue type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RequiredMappingByIssueType {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// The status IDs requiring mapping.
    #[serde(rename = "statusIds", default, skip_serializing_if = "Option::is_none")]
    pub status_ids: Option<Vec<String>>,
}

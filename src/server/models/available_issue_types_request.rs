// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailableIssueTypesRequest {
    #[serde(rename = "ignoredIssueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub ignored_issue_type_ids: Option<Vec<String>>,
}

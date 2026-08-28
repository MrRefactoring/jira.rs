// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// List of Issue Ids Or Keys that are to be archived or unarchived
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueArchivalSyncRequest {
    #[serde(rename = "issueIdsOrKeys", default, skip_serializing_if = "Option::is_none")]
    pub issue_ids_or_keys: Option<Vec<String>>,
}

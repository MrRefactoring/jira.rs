// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Number of archived/unarchived issues and list of errors that occurred during the action, if any.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueArchivalSyncResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Errors>,
    #[serde(rename = "numberOfIssuesUpdated", default, skip_serializing_if = "Option::is_none")]
    pub number_of_issues_updated: Option<i64>,
}

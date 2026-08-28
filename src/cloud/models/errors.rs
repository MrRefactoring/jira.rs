// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Errors {
    #[serde(rename = "issueIsSubtask", default, skip_serializing_if = "Option::is_none")]
    pub issue_is_subtask: Option<Error>,
    #[serde(rename = "issuesInArchivedProjects", default, skip_serializing_if = "Option::is_none")]
    pub issues_in_archived_projects: Option<Error>,
    #[serde(rename = "issuesInUnlicensedProjects", default, skip_serializing_if = "Option::is_none")]
    pub issues_in_unlicensed_projects: Option<Error>,
    #[serde(rename = "issuesNotFound", default, skip_serializing_if = "Option::is_none")]
    pub issues_not_found: Option<Error>,
    #[serde(rename = "userDoesNotHavePermission", default, skip_serializing_if = "Option::is_none")]
    pub user_does_not_have_permission: Option<Error>,
}

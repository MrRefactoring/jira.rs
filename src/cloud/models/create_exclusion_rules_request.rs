// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExclusionRulesRequest {
    /// The IDs of the issues to exclude from the plan.
    #[serde(rename = "issueIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_ids: Option<Vec<i64>>,
    /// The IDs of the issue types to exclude from the plan.
    #[serde(rename = "issueTypeIds", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<i64>>,
    /// Issues completed this number of days ago will be excluded from the plan.
    #[serde(rename = "numberOfDaysToShowCompletedIssues", default, skip_serializing_if = "Option::is_none")]
    pub number_of_days_to_show_completed_issues: Option<i64>,
    /// The IDs of the releases to exclude from the plan.
    #[serde(rename = "releaseIds", default, skip_serializing_if = "Option::is_none")]
    pub release_ids: Option<Vec<i64>>,
    /// The IDs of the work status categories to exclude from the plan.
    #[serde(rename = "workStatusCategoryIds", default, skip_serializing_if = "Option::is_none")]
    pub work_status_category_ids: Option<Vec<i64>>,
    /// The IDs of the work statuses to exclude from the plan.
    #[serde(rename = "workStatusIds", default, skip_serializing_if = "Option::is_none")]
    pub work_status_ids: Option<Vec<i64>>,
}

// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    /// The cross-project releases to include in the plan.
    #[serde(rename = "crossProjectReleases", default, skip_serializing_if = "Option::is_none")]
    pub cross_project_releases: Option<Vec<CreateCrossProjectReleaseRequest>>,
    /// The custom fields for the plan.
    #[serde(rename = "customFields", default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCustomFieldRequest>>,
    #[serde(rename = "exclusionRules", default, skip_serializing_if = "Option::is_none")]
    pub exclusion_rules: Option<CreateExclusionRulesRequest>,
    /// The issue sources to include in the plan.
    #[serde(rename = "issueSources")]
    pub issue_sources: Vec<CreateIssueSourceRequest>,
    /// The account ID of the plan lead.
    #[serde(rename = "leadAccountId", default, skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// The plan name.
    pub name: String,
    /// The permissions for the plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreatePermissionRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<CreateSchedulingRequest>,
}

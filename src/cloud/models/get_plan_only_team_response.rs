// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The planning style for the plan-only team. This is "Scrum" or "Kanban".
    pub enum GetPlanOnlyTeamResponsePlanningStyle {
        Scrum => "Scrum",
        Kanban => "Kanban",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetPlanOnlyTeamResponse {
    /// The capacity for the plan-only team.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// The plan-only team ID.
    pub id: i64,
    /// The ID of the issue source for the plan-only team.
    #[serde(rename = "issueSourceId", default, skip_serializing_if = "Option::is_none")]
    pub issue_source_id: Option<i64>,
    /// The account IDs of the plan-only team members.
    #[serde(rename = "memberAccountIds", default, skip_serializing_if = "Option::is_none")]
    pub member_account_ids: Option<Vec<String>>,
    /// The plan-only team name.
    pub name: String,
    /// The planning style for the plan-only team. This is "Scrum" or "Kanban".
    #[serde(rename = "planningStyle")]
    pub planning_style: GetPlanOnlyTeamResponsePlanningStyle,
    /// The sprint length for the plan-only team.
    #[serde(rename = "sprintLength", default, skip_serializing_if = "Option::is_none")]
    pub sprint_length: Option<i64>,
}

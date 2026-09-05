// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The planning style for the Atlassian team. This is "Scrum" or "Kanban".
    pub enum GetAtlassianTeamResponsePlanningStyle {
        Scrum => "Scrum",
        Kanban => "Kanban",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetAtlassianTeamResponse {
    /// The capacity for the Atlassian team.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// The Atlassian team ID.
    pub id: String,
    /// The ID of the issue source for the Atlassian team.
    #[serde(rename = "issueSourceId", default, skip_serializing_if = "Option::is_none")]
    pub issue_source_id: Option<i64>,
    /// The planning style for the Atlassian team. This is "Scrum" or "Kanban".
    #[serde(rename = "planningStyle")]
    pub planning_style: GetAtlassianTeamResponsePlanningStyle,
    /// The sprint length for the Atlassian team.
    #[serde(rename = "sprintLength", default, skip_serializing_if = "Option::is_none")]
    pub sprint_length: Option<i64>,
}

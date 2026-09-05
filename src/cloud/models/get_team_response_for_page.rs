// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The team type. This is "PlanOnly" or "Atlassian".
    pub enum GetTeamResponseForPageType {
        PlanOnly => "PlanOnly",
        Atlassian => "Atlassian",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetTeamResponseForPage {
    /// The team ID.
    pub id: String,
    /// The team name. This is returned if the type is "PlanOnly".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The team type. This is "PlanOnly" or "Atlassian".
    pub r#type: GetTeamResponseForPageType,
}

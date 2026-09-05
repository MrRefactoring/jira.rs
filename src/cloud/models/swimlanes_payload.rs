// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The swimlane strategy for the board.
    pub enum SwimlanesPayloadSwimlaneStrategy {
        None => "none",
        Custom => "custom",
        ParentChild => "parentChild",
        Assignee => "assignee",
        AssigneeUnassignedFirst => "assigneeUnassignedFirst",
        Epic => "epic",
        Project => "project",
        Issueparent => "issueparent",
        Issuechildren => "issuechildren",
        RequestType => "request_type",
    }
}

/// The payload for customising a swimlanes on a board
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SwimlanesPayload {
    /// The custom swimlane definitions.
    #[serde(rename = "customSwimlanes", default, skip_serializing_if = "Option::is_none")]
    pub custom_swimlanes: Option<Vec<SwimlanePayload>>,
    /// The name of the custom swimlane to use for work items that don't match any other swimlanes.
    #[serde(rename = "defaultCustomSwimlaneName", default, skip_serializing_if = "Option::is_none")]
    pub default_custom_swimlane_name: Option<String>,
    /// The swimlane strategy for the board.
    #[serde(rename = "swimlaneStrategy", default, skip_serializing_if = "Option::is_none")]
    pub swimlane_strategy: Option<SwimlanesPayloadSwimlaneStrategy>,
}

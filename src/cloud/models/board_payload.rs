// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Card color settings of the board
    pub enum BoardPayloadCardColorStrategy {
        IssueType => "ISSUE_TYPE",
        RequestType => "REQUEST_TYPE",
        Assignee => "ASSIGNEE",
        Priority => "PRIORITY",
        None => "NONE",
        Custom => "CUSTOM",
    }
}

/// The payload for creating a board
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BoardPayload {
    /// Takes in a JQL string to create a new filter. If no value is provided, it'll default to a JQL filter for the project creating
    #[serde(rename = "boardFilterJQL", default, skip_serializing_if = "Option::is_none")]
    pub board_filter_jql: Option<String>,
    /// Card color settings of the board
    #[serde(rename = "cardColorStrategy", default, skip_serializing_if = "Option::is_none")]
    pub card_color_strategy: Option<BoardPayloadCardColorStrategy>,
    #[serde(rename = "cardLayout", default, skip_serializing_if = "Option::is_none")]
    pub card_layout: Option<CardLayout>,
    /// Card layout settings of the board
    #[serde(rename = "cardLayouts", default, skip_serializing_if = "Option::is_none")]
    pub card_layouts: Option<Vec<CardLayoutField>>,
    /// The columns of the board
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<BoardColumnPayload>>,
    /// Whether to enable the card cover option on this board
    #[serde(rename = "enableCardCover", default, skip_serializing_if = "Option::is_none")]
    pub enable_card_cover: Option<bool>,
    /// The name of the board
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// The quick filters for the board.
    #[serde(rename = "quickFilters", default, skip_serializing_if = "Option::is_none")]
    pub quick_filters: Option<Vec<QuickFilterPayload>>,
    /// Whether sprints are supported on the board
    #[serde(rename = "supportsSprint", default, skip_serializing_if = "Option::is_none")]
    pub supports_sprint: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swimlanes: Option<SwimlanesPayload>,
    #[serde(rename = "workingDaysConfig", default, skip_serializing_if = "Option::is_none")]
    pub working_days_config: Option<WorkingDaysConfig>,
}

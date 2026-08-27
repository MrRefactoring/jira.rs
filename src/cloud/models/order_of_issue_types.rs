// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The position the issue types should be moved to. Required if `after` isn't provided.
    pub enum OrderOfIssueTypesPosition {
        First => "First",
        Last => "Last",
    }
}

/// An ordered list of issue type IDs and information about where to move them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderOfIssueTypes {
    /// The ID of the issue type to place the moved issue types after. Required if `position` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A list of the issue type IDs to move. The order of the issue type IDs in the list is the order they are given after the move.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
    /// The position the issue types should be moved to. Required if `after` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<OrderOfIssueTypesPosition>,
}

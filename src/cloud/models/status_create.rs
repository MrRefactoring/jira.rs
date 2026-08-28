// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The category of the status.
    pub enum StatusCreateStatusCategory {
        Todo => "TODO",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
    }
}

/// Details of the status being created.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusCreate {
    /// The description of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the status.
    pub name: String,
    /// The category of the status.
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCreateStatusCategory,
}

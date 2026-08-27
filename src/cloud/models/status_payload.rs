// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The conflict strategy for the status already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - Create a new entity
    pub enum StatusPayloadOnConflict {
        Fail => "FAIL",
        Use => "USE",
        New => "NEW",
    }
}

crate::open_enum! {
    /// The status category of the status. The value is case-sensitive.
    pub enum StatusPayloadStatusCategory {
        Todo => "TODO",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
    }
}

/// The payload for creating a status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusPayload {
    /// The description of the status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The conflict strategy for the status already exists. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - Create a new entity
    #[serde(rename = "onConflict", default, skip_serializing_if = "Option::is_none")]
    pub on_conflict: Option<StatusPayloadOnConflict>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    /// The status category of the status. The value is case-sensitive.
    #[serde(rename = "statusCategory", default, skip_serializing_if = "Option::is_none")]
    pub status_category: Option<StatusPayloadStatusCategory>,
}

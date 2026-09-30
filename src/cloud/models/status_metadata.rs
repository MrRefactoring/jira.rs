// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The category of the status.
    pub enum StatusMetadataCategory {
        Todo => "TODO",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
    }
}

/// The details of the statuses in the associated workflows.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StatusMetadata {
    /// The category of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<StatusMetadataCategory>,
    /// The ID of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

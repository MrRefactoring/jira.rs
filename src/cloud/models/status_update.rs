// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The category of the status.
    pub enum StatusUpdateStatusCategory {
        Todo => "TODO",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
    }
}

/// Details of the status being updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusUpdate {
    /// The description of the status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the status.
    pub id: String,
    /// The name of the status.
    pub name: String,
    /// The category of the status.
    #[serde(rename = "statusCategory")]
    pub status_category: StatusUpdateStatusCategory,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for StatusUpdate {
    const FIELDS: &'static [&'static str] = &["description", "id", "name", "statusCategory"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

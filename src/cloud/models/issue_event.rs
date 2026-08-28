// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about an issue event.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueEvent {
    /// The ID of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

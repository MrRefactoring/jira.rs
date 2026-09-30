// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The event ID to use for reference in the payload
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEventIDPayload {
    /// The event ID to use for reference in the payload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

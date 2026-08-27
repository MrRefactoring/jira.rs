// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The ID of an event that is being mapped to notifications.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEventTypeId {
    /// The ID of the notification scheme event.
    pub id: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

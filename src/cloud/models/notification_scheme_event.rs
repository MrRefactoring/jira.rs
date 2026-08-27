// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about a notification scheme event.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<NotificationEvent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<EventNotification>>,
}

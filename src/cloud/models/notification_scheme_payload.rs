// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The strategy to use when there is a conflict with an existing entity
    pub enum NotificationSchemePayloadOnConflict {
        Fail => "FAIL",
        Use => "USE",
        New => "NEW",
    }
}

/// The payload for creating a notification scheme. The user has to supply the ID for the default notification scheme. For CMP this is provided in the project payload and should be left empty, for TMP it's provided using this payload
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemePayload {
    /// The description of the notification scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the notification scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The events and notifications for the notification scheme
    #[serde(rename = "notificationSchemeEvents", default, skip_serializing_if = "Option::is_none")]
    pub notification_scheme_events: Option<Vec<NotificationSchemeEventPayload>>,
    /// The strategy to use when there is a conflict with an existing entity
    #[serde(rename = "onConflict", default, skip_serializing_if = "Option::is_none")]
    pub on_conflict: Option<NotificationSchemePayloadOnConflict>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
}

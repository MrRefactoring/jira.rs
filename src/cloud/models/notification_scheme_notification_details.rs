// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of a notification within a notification scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeNotificationDetails {
    /// The notification type, e.g `CurrentAssignee`, `Group`, `EmailAddress`.
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    /// The value corresponding to the specified notification type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for NotificationSchemeNotificationDetails {
    const FIELDS: &'static [&'static str] = &["notificationType", "parameter"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

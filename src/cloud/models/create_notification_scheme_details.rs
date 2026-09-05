// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of an notification scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateNotificationSchemeDetails {
    /// The description of the notification scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the notification scheme. Must be unique (case-insensitive).
    pub name: String,
    /// The list of notifications which should be added to the notification scheme.
    #[serde(rename = "notificationSchemeEvents", default, skip_serializing_if = "Option::is_none")]
    pub notification_scheme_events: Option<Vec<NotificationSchemeEventDetails>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for CreateNotificationSchemeDetails {
    const FIELDS: &'static [&'static str] = &["description", "name", "notificationSchemeEvents"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

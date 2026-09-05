// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a notification scheme event.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEventDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<NotificationSchemeEventTypeId>,
    /// The list of notifications mapped to a specified event.
    pub notifications: Vec<NotificationSchemeNotificationDetails>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for NotificationSchemeEventDetails {
    const FIELDS: &'static [&'static str] = &["event", "notifications"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

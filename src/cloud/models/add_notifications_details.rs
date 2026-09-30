// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of notifications which should be added to the notification scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddNotificationsDetails {
    /// The list of notifications which should be added to the notification scheme.
    #[serde(rename = "notificationSchemeEvents")]
    pub notification_scheme_events: Vec<NotificationSchemeEventDetails>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for AddNotificationsDetails {
    const FIELDS: &'static [&'static str] = &["notificationSchemeEvents"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

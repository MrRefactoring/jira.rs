// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The ID of a notification scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NotificationSchemeId {
    /// The ID of a notification scheme.
    pub id: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for NotificationSchemeId {
    const FIELDS: &'static [&'static str] = &["id"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

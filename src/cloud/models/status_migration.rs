// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The mapping of old to new status ID.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusMigration {
    /// The new status ID.
    #[serde(rename = "newStatusReference")]
    pub new_status_reference: String,
    /// The old status ID.
    #[serde(rename = "oldStatusReference")]
    pub old_status_reference: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for StatusMigration {
    const FIELDS: &'static [&'static str] = &["newStatusReference", "oldStatusReference"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

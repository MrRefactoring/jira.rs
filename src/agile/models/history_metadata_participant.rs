// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HistoryMetadataParticipant {
    /// The URL to an avatar for the user or system associated with a history record.
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user or system associated with a history record.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The key of the display name of the user or system associated with a history record.
    #[serde(rename = "displayNameKey", default, skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    /// The ID of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL of the user or system associated with a history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for HistoryMetadataParticipant {
    const FIELDS: &'static [&'static str] = &["avatarUrl", "displayName", "displayNameKey", "id", "type", "url"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

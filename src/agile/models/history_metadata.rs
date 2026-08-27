// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryMetadataActor {
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
}

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryMetadataCause {
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
}

/// Details of user or system associated with a issue history metadata item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryMetadataGenerator {
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
}

/// Details of issue history metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryMetadata {
    /// The activity described in the history record.
    #[serde(rename = "activityDescription", default, skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    /// The key of the activity described in the history record.
    #[serde(rename = "activityDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<HistoryMetadataActor>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<HistoryMetadataCause>,
    /// The description of the history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description key of the history record.
    #[serde(rename = "descriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    /// The description of the email address associated the history record.
    #[serde(rename = "emailDescription", default, skip_serializing_if = "Option::is_none")]
    pub email_description: Option<String>,
    /// The description key of the email address associated the history record.
    #[serde(rename = "emailDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub email_description_key: Option<String>,
    /// Additional arbitrary information about the history record.
    #[serde(rename = "extraData", default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Details of user or system associated with a issue history metadata item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<HistoryMetadataGenerator>,
    /// The type of the history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

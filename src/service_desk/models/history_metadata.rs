// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of issue history metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HistoryMetadata {
    /// The activity described in the history record.
    #[serde(rename = "activityDescription", default, skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    /// The key of the activity described in the history record.
    #[serde(rename = "activityDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<HistoryMetadataParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<HistoryMetadataParticipant>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<HistoryMetadataParticipant>,
    /// The type of the history record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

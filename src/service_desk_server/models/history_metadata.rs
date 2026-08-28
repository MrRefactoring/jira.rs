// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HistoryMetadata {
    #[serde(rename = "activityDescription", default, skip_serializing_if = "Option::is_none")]
    pub activity_description: Option<String>,
    #[serde(rename = "activityDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub activity_description_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<HistoryMetadataParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<HistoryMetadataParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub description_key: Option<String>,
    #[serde(rename = "emailDescription", default, skip_serializing_if = "Option::is_none")]
    pub email_description: Option<String>,
    #[serde(rename = "emailDescriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub email_description_key: Option<String>,
    #[serde(rename = "extraData", default, skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<HistoryMetadataParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

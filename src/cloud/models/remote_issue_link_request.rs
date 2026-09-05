// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a remote issue link.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteIssueLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    /// An identifier for the remote item in the remote system. For example, the global ID for a remote item in Confluence would consist of the app ID and page ID, like this: `appId=456&pageId=123`.
    ///
    /// Setting this field enables the remote issue link details to be updated or deleted using remote system and item details as the record identifier, rather than using the record's Jira ID.
    ///
    /// The maximum length is 255 characters.
    #[serde(rename = "globalId", default, skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RemoteObject>,
    /// Description of the relationship between the issue and the linked item. If not set, the relationship description "links to" is used in Jira.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for RemoteIssueLinkRequest {
    const FIELDS: &'static [&'static str] = &["application", "globalId", "object", "relationship"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

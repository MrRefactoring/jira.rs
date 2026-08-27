// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookInput {
    pub name: String,
    /// Where Jira posts the event.
    pub url: String,
    /// The events to deliver, e.g. `jira:issue_created`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    /// Narrows what is delivered, e.g. `{ "issue-related-events-section": jql }`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Deliver the event without its body.
    #[serde(rename = "excludeBody", default, skip_serializing_if = "Option::is_none")]
    pub exclude_body: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "sslVerificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub ssl_verification_required: Option<bool>,
}

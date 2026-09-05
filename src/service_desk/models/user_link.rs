// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserLink {
    /// Links to the various sizes of the customer's avatar. Note that this property is deprecated, and will be removed in future versions.
    #[deprecated(note = "Note that this property is deprecated, and will be removed in future versions.")]
    #[serde(rename = "avatarUrls", default, skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// REST API URL for the customer.
    #[serde(rename = "jiraRest", default, skip_serializing_if = "Option::is_none")]
    pub jira_rest: Option<String>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

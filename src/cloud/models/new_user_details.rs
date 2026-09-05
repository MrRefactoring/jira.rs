// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The user details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NewUserDetails {
    /// The email address for the user.
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    /// Products the new user has access to. Valid products are: jira-core, jira-servicedesk, jira-product-discovery, jira-software. To create a user without product access, set this field to be an empty array.
    pub products: Vec<String>,
    /// The URL of the user.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for NewUserDetails {
    const FIELDS: &'static [&'static str] = &["emailAddress", "products", "self"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}

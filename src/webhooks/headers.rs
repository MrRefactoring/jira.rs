use serde::{Deserialize, Serialize};

pub const IDENTIFIER: &str = "x-atlassian-webhook-identifier";
pub const FLOW: &str = "x-atlassian-webhook-flow";
pub const RETRY: &str = "x-atlassian-webhook-retry";
pub const TRACE: &str = "x-atlassian-webhook-trace";
pub const SIGNATURE: &str = "x-hub-signature";

crate::open_enum! {
    pub enum WebhookFlow {
        Primary => "Primary",
        Secondary => "Secondary",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookHeaders {
    #[serde(rename = "x-atlassian-webhook-identifier")]
    pub identifier: Option<String>,
    #[serde(rename = "x-atlassian-webhook-flow")]
    pub flow: Option<WebhookFlow>,
    #[serde(rename = "x-atlassian-webhook-retry")]
    pub retry: Option<String>,
    #[serde(rename = "x-atlassian-webhook-trace")]
    pub trace: Option<String>,
    #[serde(rename = "x-hub-signature")]
    pub signature: Option<String>,
}

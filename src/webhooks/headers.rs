//! The headers Jira attaches to a webhook it delivers.
//!
//! Written in lower case, because that is how they arrive.

use serde::{Deserialize, Serialize};

/// Identifies this delivery. Unique within the site and unchanged across retries, so it is what to
/// record to recognise a webhook already handled.
pub const IDENTIFIER: &str = "x-atlassian-webhook-identifier";
/// Which delivery lane carried it. See [`WebhookFlow`].
pub const FLOW: &str = "x-atlassian-webhook-flow";
/// How many times this delivery has been retried. Absent on the first attempt, and a string: an HTTP
/// header has no numbers in it.
pub const RETRY: &str = "x-atlassian-webhook-retry";
/// Whatever a Connect app attached to the REST request that caused the event. Absent unless an app set it.
pub const TRACE: &str = "x-atlassian-webhook-trace";
/// The body's signature, as `method=signature`. Present only on a webhook registered with a secret, and
/// what [`verify_signature`](super::verify_signature) takes.
pub const SIGNATURE: &str = "x-hub-signature";

crate::open_enum! {
    /// Which delivery lane carried an event.
    ///
    /// `Primary` is the event itself and should arrive within thirty seconds. `Secondary` is the fallout of a bulk or
    /// cascading change — deleting an issue sends `jira:issue_deleted` as primary and every dependent
    /// `comment_deleted` and `attachment_deleted` as secondary — and is allowed a quarter of an hour.
    pub enum WebhookFlow {
        Primary => "Primary",
        Secondary => "Secondary",
    }
}

/// The headers of one delivery, for a server that would rather deserialize them than index a map.
///
/// Every field is optional: only the identifier and the flow arrive on every delivery, and a receiver that demands
/// the others rejects perfectly ordinary traffic.
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

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about a failed webhook.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FailedWebhook {
    /// The webhook body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// The time the webhook was added to the list of failed webhooks (that is, the time of the last failed retry).
    #[serde(rename = "failureTime")]
    pub failure_time: i64,
    /// The webhook ID, as sent in the `X-Atlassian-Webhook-Identifier` header with the webhook.
    pub id: String,
    /// The original webhook destination.
    pub url: String,
}

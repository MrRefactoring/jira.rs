// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// ID of a registered webhook or error messages explaining why a webhook wasn't registered.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RegisteredWebhook {
    /// The ID of the webhook. Returned if the webhook is created.
    #[serde(rename = "createdWebhookId", default, skip_serializing_if = "Option::is_none")]
    pub created_webhook_id: Option<i64>,
    /// Error messages specifying why the webhook creation failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

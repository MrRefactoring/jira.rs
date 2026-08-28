// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Container for a list of registered webhooks. Webhook details are returned in the same order as the request.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContainerForRegisteredWebhooks {
    /// A list of registered webhooks.
    #[serde(rename = "webhookRegistrationResult", default, skip_serializing_if = "Option::is_none")]
    pub webhook_registration_result: Option<Vec<RegisteredWebhook>>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The date the refreshed webhooks expire.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhooksExpirationDate {
    /// The expiration date of all the refreshed webhooks.
    #[serde(rename = "expirationDate")]
    pub expiration_date: i64,
}

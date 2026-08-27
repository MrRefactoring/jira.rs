// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestNotificationSubscription {
    /// Indicates whether the user is subscribed (true) or not (false) to the request's notifications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
}

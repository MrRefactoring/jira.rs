// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteApiRequest {
    pub email: String,
    #[serde(rename = "permissionRule", default, skip_serializing_if = "Option::is_none")]
    pub permission_rule: Option<RoleAssociation>,
    #[serde(rename = "sendNotification", default, skip_serializing_if = "Option::is_none")]
    pub send_notification: Option<bool>,
    #[serde(rename = "notificationText", default, skip_serializing_if = "Option::is_none")]
    pub notification_text: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationSchemeAndProjectMapping {
    #[serde(rename = "notificationSchemeId", default, skip_serializing_if = "Option::is_none")]
    pub notification_scheme_id: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAnonymizationRerunRequest {
    #[serde(rename = "newOwnerKey", default, skip_serializing_if = "Option::is_none")]
    pub new_owner_key: Option<String>,
    #[serde(rename = "oldUserKey", default, skip_serializing_if = "Option::is_none")]
    pub old_user_key: Option<String>,
    #[serde(rename = "oldUserName", default, skip_serializing_if = "Option::is_none")]
    pub old_user_name: Option<String>,
    #[serde(rename = "userKey", default, skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
}

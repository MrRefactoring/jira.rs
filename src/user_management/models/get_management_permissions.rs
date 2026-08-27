// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetManagementPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<ManageabilityRuleObjectMutability>,
    #[serde(rename = "profile.write", default, skip_serializing_if = "Option::is_none")]
    pub profile_write: Option<ManageabilityRuleObjectMutability>,
    #[serde(rename = "profile.read", default, skip_serializing_if = "Option::is_none")]
    pub profile_read: Option<ManageabilityRuleSimple>,
    #[serde(rename = "email.set", default, skip_serializing_if = "Option::is_none")]
    pub email_set: Option<ManageabilityRuleSimple>,
    #[serde(rename = "lifecycle.enablement", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_enablement: Option<ManageabilityRuleSimple>,
    #[serde(rename = "lifecycle.delete", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_delete: Option<ManageabilityRuleSimple>,
    #[serde(rename = "apiToken.read", default, skip_serializing_if = "Option::is_none")]
    pub api_token_read: Option<ManageabilityRuleSimple>,
    #[serde(rename = "apiToken.delete", default, skip_serializing_if = "Option::is_none")]
    pub api_token_delete: Option<ManageabilityRuleSimple>,
}

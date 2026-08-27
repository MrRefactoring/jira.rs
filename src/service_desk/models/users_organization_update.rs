// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsersOrganizationUpdate {
    /// List of customers, specific by account IDs, to add to or remove from the organization.
    #[serde(rename = "accountIds", default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// The organizationId in which users need to be added
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<i64>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. Use `accountIds` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

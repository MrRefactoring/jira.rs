// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestParticipantUpdate {
    /// List of users, specified by account IDs, to add to or remove as participants in the request.
    #[serde(rename = "accountIds", default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. Use `accountIds` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,
}

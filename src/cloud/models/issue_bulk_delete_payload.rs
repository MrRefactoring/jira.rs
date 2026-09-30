// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Issue Bulk Delete Payload
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueBulkDeletePayload {
    /// List of issue IDs or keys which are to be bulk deleted. These IDs or keys can be from different projects and issue types.
    #[serde(rename = "selectedIssueIdsOrKeys")]
    pub selected_issue_ids_or_keys: Vec<String>,
    /// A boolean value that indicates whether to send a bulk change notification when the issues are being deleted.
    ///
    /// If `true`, dispatches a bulk notification email to users about the updates.
    #[serde(rename = "sendBulkNotification", default, skip_serializing_if = "Option::is_none")]
    pub send_bulk_notification: Option<bool>,
}

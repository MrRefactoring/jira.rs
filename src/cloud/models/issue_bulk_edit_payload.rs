// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue Bulk Edit Payload
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueBulkEditPayload {
    #[serde(rename = "editedFieldsInput", default, skip_serializing_if = "Option::is_none")]
    pub edited_fields_input: Option<JiraIssueFields>,
    /// List of all the field IDs that are to be bulk edited. Each field ID in this list corresponds to a specific attribute of an issue that is set to be modified in the bulk edit operation. The relevant field ID can be obtained by calling the Bulk Edit Get Fields REST API (documentation available on this page itself).
    #[serde(rename = "selectedActions")]
    pub selected_actions: Vec<String>,
    /// List of issue IDs or keys which are to be bulk edited. These IDs or keys can be from different projects and issue types.
    #[serde(rename = "selectedIssueIdsOrKeys")]
    pub selected_issue_ids_or_keys: Vec<String>,
    /// A boolean value that indicates whether to send a bulk change notification when the issues are being edited.
    ///
    /// If `true`, dispatches a bulk notification email to users about the updates.
    #[serde(rename = "sendBulkNotification", default, skip_serializing_if = "Option::is_none")]
    pub send_bulk_notification: Option<bool>,
}

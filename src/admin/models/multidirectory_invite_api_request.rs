// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MultidirectoryInviteApiRequest {
    /// Email addresses of the people you want to invite.
    pub emails: Vec<String>,
    /// The access you want to give users from this invitation.
    #[serde(rename = "permissionRules", default, skip_serializing_if = "Option::is_none")]
    pub permission_rules: Option<Vec<MultidirectoryInviteRoleAssociation>>,
    /// The groups you want to add users to from this invitation. Use the [Get groups in an organization](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/#api-v2-orgs-orgid-directories-directoryid-groups-get) endpoint to find the group IDs.
    #[serde(rename = "additionalGroups", default, skip_serializing_if = "Option::is_none")]
    pub additional_groups: Option<Vec<String>>,
    /// Set to true if you want to send an email invitation.
    #[serde(rename = "sendNotification", default, skip_serializing_if = "Option::is_none")]
    pub send_notification: Option<bool>,
    /// Add a message to your email invitation.
    #[serde(rename = "notificationText", default, skip_serializing_if = "Option::is_none")]
    pub notification_text: Option<String>,
}

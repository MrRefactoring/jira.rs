// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Identifies the recipients of the notification.
    pub enum EventNotificationNotificationType {
        CurrentAssignee => "CurrentAssignee",
        Reporter => "Reporter",
        CurrentUser => "CurrentUser",
        ProjectLead => "ProjectLead",
        ComponentLead => "ComponentLead",
        User => "User",
        Group => "Group",
        ProjectRole => "ProjectRole",
        EmailAddress => "EmailAddress",
        AllWatchers => "AllWatchers",
        UserCustomField => "UserCustomField",
        GroupCustomField => "GroupCustomField",
    }
}

/// Details about a notification associated with an event.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventNotification {
    /// The email address.
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Expand options that include additional event notification details in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupName>,
    /// The ID of the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Identifies the recipients of the notification.
    #[serde(rename = "notificationType", default, skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<EventNotificationNotificationType>,
    /// As a group's name can change, use of `recipient` is recommended. The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by `notificationType` value. So, when `notificationType` is:
    ///
    ///  *  `User` The `parameter` is the user account ID.
    ///  *  `Group` The `parameter` is the group name.
    ///  *  `ProjectRole` The `parameter` is the project role ID.
    ///  *  `UserCustomField` The `parameter` is the ID of the custom field.
    ///  *  `GroupCustomField` The `parameter` is the ID of the custom field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "projectRole", default, skip_serializing_if = "Option::is_none")]
    pub project_role: Option<ProjectRole>,
    /// The identifier associated with the `notificationType` value that defines the receiver of the notification, where the receiver isn't implied by the `notificationType` value. So, when `notificationType` is:
    ///
    ///  *  `User`, `recipient` is the user account ID.
    ///  *  `Group`, `recipient` is the group ID.
    ///  *  `ProjectRole`, `recipient` is the project role ID.
    ///  *  `UserCustomField`, `recipient` is the ID of the custom field.
    ///  *  `GroupCustomField`, `recipient` is the ID of the custom field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserDetails>,
}

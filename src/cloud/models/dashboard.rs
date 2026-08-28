// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of a dashboard.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Dashboard {
    /// The automatic refresh interval for the dashboard in milliseconds.
    #[serde(rename = "automaticRefreshMs", default, skip_serializing_if = "Option::is_none")]
    pub automatic_refresh_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The details of any edit share permissions for the dashboard.
    #[serde(rename = "editPermissions", default, skip_serializing_if = "Option::is_none")]
    pub edit_permissions: Option<Vec<SharePermission>>,
    /// The ID of the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the dashboard is selected as a favorite by the user.
    #[serde(rename = "isFavourite", default, skip_serializing_if = "Option::is_none")]
    pub is_favourite: Option<bool>,
    /// Whether the current user has permission to edit the dashboard.
    #[serde(rename = "isWritable", default, skip_serializing_if = "Option::is_none")]
    pub is_writable: Option<bool>,
    /// The name of the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserBean>,
    /// The number of users who have this dashboard as a favorite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i64>,
    /// The rank of this dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// The URL of these dashboard details.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// The details of any view share permissions for the dashboard.
    #[serde(rename = "sharePermissions", default, skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<SharePermission>>,
    /// Whether the current dashboard is system dashboard.
    #[serde(rename = "systemDashboard", default, skip_serializing_if = "Option::is_none")]
    pub system_dashboard: Option<bool>,
    /// The URL of the dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
}

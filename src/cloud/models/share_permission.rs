// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of share permission:
    ///
    ///  *  `user` Shared with a user.
    ///  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.
    ///  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.
    ///  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.
    ///  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.
    ///  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.
    ///  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
    pub enum SharePermissionType {
        User => "user",
        Group => "group",
        Project => "project",
        ProjectRole => "projectRole",
        Global => "global",
        Loggedin => "loggedin",
        Authenticated => "authenticated",
        ProjectUnknown => "project-unknown",
    }
}

/// Details of a share permission for the filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharePermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupName>,
    /// The unique identifier of the share permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<Project>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ProjectRole>,
    /// The type of share permission:
    ///
    ///  *  `user` Shared with a user.
    ///  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.
    ///  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.
    ///  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.
    ///  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.
    ///  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.
    ///  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
    pub r#type: SharePermissionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserBean>,
}

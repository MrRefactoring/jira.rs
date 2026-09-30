// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// A list of permission keys. (Required) This parameter accepts a comma-separated list. To get the list of available permissions, use [Get all permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permissions/#api-rest-api-3-permissions-get).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetMyPermissionsRequestPermissions {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Permissions operations.
pub struct PermissionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PermissionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of permissions indicating which permissions the user has. Details of the user's permissions can be obtained in a global, project, issue or comment context.
    ///
    /// The user is reported as having a project permission:
    ///
    ///  *  in the global context, if the user has the project permission in any project.
    ///  *  for a project, where the project permission is determined using issue data, if the user meets the permission's criteria for any issue in the project. Otherwise, if the user has the project permission in the project.
    ///  *  for an issue, where a project permission is determined using issue data, if the user has the permission in the issue. Otherwise, if the user has the project permission in the project containing the issue.
    ///  *  for a comment, where the user has both the permission to browse the comment and the project permission for the comment's parent issue. Only the BROWSE\_PROJECTS permission is supported. If a `commentId` is provided whose `permissions` does not equal BROWSE\_PROJECTS, a 400 error will be returned.
    ///
    /// This means that users may be shown as having an issue permission (such as EDIT\_ISSUES) in the global context or a project context but may not have the permission for any or all issues. For example, if Reporters have the EDIT\_ISSUES permission a user would be shown as having this permission in the global context or the context of a project, because any user can be a reporter. However, if they are not the user who reported the issue queried they would not have EDIT\_ISSUES permission for that issue.
    ///
    /// For [Jira Service Management project permissions](https://support.atlassian.com/jira-cloud-administration/docs/customize-jira-service-management-permissions/), this will be evaluated similarly to a user in the customer portal. For example, if the BROWSE\_PROJECTS permission is granted to Service Project Customer - Portal Access, any users with access to the customer portal will have the BROWSE\_PROJECTS permission.
    ///
    /// Global permissions are unaffected by context.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_my_permissions(&self) -> GetMyPermissionsRequest<'a> {
        GetMyPermissionsRequest::new(self.client)
    }

    /// Returns all permissions, including:
    ///
    ///  *  global permissions.
    ///  *  project permissions.
    ///  *  global permissions added by plugins.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_all_permissions(&self) -> GetAllPermissionsRequest<'a> {
        GetAllPermissionsRequest::new(self.client)
    }

    /// Returns:
    ///
    ///  *  for a list of global permissions, the global permissions granted to a user.
    ///  *  for a list of project permissions and lists of projects and issues, for each project permission a list of the projects and issues a user can access or manipulate.
    ///
    /// If no account ID is provided, the operation returns details for the logged in user.
    ///
    /// Note that:
    ///
    ///  *  Invalid project and issue IDs are ignored.
    ///  *  A maximum of 1000 projects and 1000 issues can be checked.
    ///  *  Null values in `globalPermissions`, `projectPermissions`, `projectPermissions.projects`, and `projectPermissions.issues` are ignored.
    ///  *  Empty strings in `projectPermissions.permissions` are ignored.
    ///
    /// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
    ///
    ///  *  **Classic**: `read:jira-work`
    ///  *  **Granular**: `read:permission:jira`
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to check the permissions for other users, otherwise none. However, Connect apps can make a call from the app server to the product to obtain permission details for any user, without admin permission. This Connect app ability doesn't apply to calls made using AP.request() in a browser.
    pub fn get_bulk_permissions(
        &self,
        bulk_permissions_request: BulkPermissionsRequest,
    ) -> GetBulkPermissionsRequest<'a> {
        GetBulkPermissionsRequest::new(self.client, bulk_permissions_request)
    }

    /// Returns all the projects where the user is granted a list of project permissions.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_permitted_projects(&self, permissions_keys: PermissionsKeys) -> GetPermittedProjectsRequest<'a> {
        GetPermittedProjectsRequest::new(self.client, permissions_keys)
    }
}

/// Returns a list of permissions indicating which permissions the user has. Details of the user's permissions can be obtained in a global, project, issue or comment context.
///
/// The user is reported as having a project permission:
///
///  *  in the global context, if the user has the project permission in any project.
///  *  for a project, where the project permission is determined using issue data, if the user meets the permission's criteria for any issue in the project. Otherwise, if the user has the project permission in the project.
///  *  for an issue, where a project permission is determined using issue data, if the user has the permission in the issue. Otherwise, if the user has the project permission in the project containing the issue.
///  *  for a comment, where the user has both the permission to browse the comment and the project permission for the comment's parent issue. Only the BROWSE\_PROJECTS permission is supported. If a `commentId` is provided whose `permissions` does not equal BROWSE\_PROJECTS, a 400 error will be returned.
///
/// This means that users may be shown as having an issue permission (such as EDIT\_ISSUES) in the global context or a project context but may not have the permission for any or all issues. For example, if Reporters have the EDIT\_ISSUES permission a user would be shown as having this permission in the global context or the context of a project, because any user can be a reporter. However, if they are not the user who reported the issue queried they would not have EDIT\_ISSUES permission for that issue.
///
/// For [Jira Service Management project permissions](https://support.atlassian.com/jira-cloud-administration/docs/customize-jira-service-management-permissions/), this will be evaluated similarly to a user in the customer portal. For example, if the BROWSE\_PROJECTS permission is granted to Service Project Customer - Portal Access, any users with access to the customer portal will have the BROWSE\_PROJECTS permission.
///
/// Global permissions are unaffected by context.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetMyPermissionsRequest<'a> {
    client: &'a crate::core::Client,
    project_key: Option<String>,
    project_id: Option<String>,
    issue_key: Option<String>,
    issue_id: Option<String>,
    permissions: Option<GetMyPermissionsRequestPermissions>,
    project_uuid: Option<String>,
    project_configuration_uuid: Option<String>,
    comment_id: Option<String>,
}

impl<'a> GetMyPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            project_key: None,
            project_id: None,
            issue_key: None,
            issue_id: None,
            permissions: None,
            project_uuid: None,
            project_configuration_uuid: None,
            comment_id: None,
        }
    }

    /// The key of project. Ignored if `projectId` is provided.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// The ID of project.
    #[must_use]
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());

        self
    }

    /// The key of the issue. Ignored if `issueId` is provided.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The ID of the issue.
    #[must_use]
    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());

        self
    }

    /// A list of permission keys. (Required) This parameter accepts a comma-separated list. To get the list of available permissions, use [Get all permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-permissions/#api-rest-api-3-permissions-get).
    #[must_use]
    pub fn permissions(mut self, value: GetMyPermissionsRequestPermissions) -> Self {
        self.permissions = Some(value);

        self
    }

    #[must_use]
    pub fn project_uuid(mut self, value: impl Into<String>) -> Self {
        self.project_uuid = Some(value.into());

        self
    }

    #[must_use]
    pub fn project_configuration_uuid(mut self, value: impl Into<String>) -> Self {
        self.project_configuration_uuid = Some(value.into());

        self
    }

    /// The ID of the comment.
    #[must_use]
    pub fn comment_id(mut self, value: impl Into<String>) -> Self {
        self.comment_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/mypermissions".to_owned());

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_id {
            config.query.push(("issueId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.permissions {
            config.query.push(("permissions".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.project_uuid {
            config.query.push(("projectUuid".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_configuration_uuid {
            config.query.push(("projectConfigurationUuid".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.comment_id {
            config.query.push(("commentId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Permissions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all permissions, including:
///
///  *  global permissions.
///  *  project permissions.
///  *  global permissions added by plugins.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetAllPermissionsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/permissions".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Permissions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns:
///
///  *  for a list of global permissions, the global permissions granted to a user.
///  *  for a list of project permissions and lists of projects and issues, for each project permission a list of the projects and issues a user can access or manipulate.
///
/// If no account ID is provided, the operation returns details for the logged in user.
///
/// Note that:
///
///  *  Invalid project and issue IDs are ignored.
///  *  A maximum of 1000 projects and 1000 issues can be checked.
///  *  Null values in `globalPermissions`, `projectPermissions`, `projectPermissions.projects`, and `projectPermissions.issues` are ignored.
///  *  Empty strings in `projectPermissions.permissions` are ignored.
///
/// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
///
///  *  **Classic**: `read:jira-work`
///  *  **Granular**: `read:permission:jira`
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to check the permissions for other users, otherwise none. However, Connect apps can make a call from the app server to the product to obtain permission details for any user, without admin permission. This Connect app ability doesn't apply to calls made using AP.request() in a browser.
#[derive(Clone)]
pub struct GetBulkPermissionsRequest<'a> {
    client: &'a crate::core::Client,
    bulk_permissions_request: BulkPermissionsRequest,
}

impl<'a> GetBulkPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client, bulk_permissions_request: BulkPermissionsRequest) -> Self {
        Self { client, bulk_permissions_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/permissions/check".to_owned());

        let body = match serde_json::to_value(&self.bulk_permissions_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkPermissionGrants> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all the projects where the user is granted a list of project permissions.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetPermittedProjectsRequest<'a> {
    client: &'a crate::core::Client,
    permissions_keys: PermissionsKeys,
}

impl<'a> GetPermittedProjectsRequest<'a> {
    fn new(client: &'a crate::core::Client, permissions_keys: PermissionsKeys) -> Self {
        Self { client, permissions_keys }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/permissions/project".to_owned());

        let body = match serde_json::to_value(&self.permissions_keys)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermittedProjects> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

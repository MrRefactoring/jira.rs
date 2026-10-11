// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    pub enum GetAssignedPermissionSchemeRequestExpandValue {
        All => "all",
        Field => "field",
        Group => "group",
        Permissions => "permissions",
        ProjectRole => "projectRole",
        User => "user",
    }
}

crate::open_enum! {
    pub enum AssignPermissionSchemeRequestExpandValue {
        All => "all",
        Field => "field",
        Group => "group",
        Permissions => "permissions",
        ProjectRole => "projectRole",
        User => "user",
    }
}

/// The ProjectPermissionSchemes operations.
pub struct ProjectPermissionSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectPermissionSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the [issue security scheme](https://confluence.atlassian.com/x/J4lKLg) associated with the project.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or the *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn get_project_issue_security_scheme(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetProjectIssueSecuritySchemeRequest<'a> {
        GetProjectIssueSecuritySchemeRequest::new(self.client, project_key_or_id)
    }

    /// Gets the [permission scheme](https://confluence.atlassian.com/x/yodKLg) associated with the project.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
    pub fn get_assigned_permission_scheme(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetAssignedPermissionSchemeRequest<'a> {
        GetAssignedPermissionSchemeRequest::new(self.client, project_key_or_id)
    }

    /// Assigns a permission scheme with a project. See [Managing project permissions](https://confluence.atlassian.com/x/yodKLg) for more information about permission schemes.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg)
    pub fn assign_permission_scheme(
        &self,
        project_key_or_id: impl Into<String>,
        id: Id,
    ) -> AssignPermissionSchemeRequest<'a> {
        AssignPermissionSchemeRequest::new(self.client, project_key_or_id, id)
    }

    /// Returns all [issue security](https://confluence.atlassian.com/x/J4lKLg) levels for the project that the user has access to.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project, however, issue security levels are only returned for authenticated user with *Set Issue Security* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project.
    pub fn get_security_levels_for_project(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetSecurityLevelsForProjectRequest<'a> {
        GetSecurityLevelsForProjectRequest::new(self.client, project_key_or_id)
    }
}

/// Returns the [issue security scheme](https://confluence.atlassian.com/x/J4lKLg) associated with the project.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or the *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
#[derive(Clone)]
pub struct GetProjectIssueSecuritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
}

impl<'a> GetProjectIssueSecuritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/project/{}/issuesecuritylevelscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SecurityScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Gets the [permission scheme](https://confluence.atlassian.com/x/yodKLg) associated with the project.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg).
#[derive(Clone)]
pub struct GetAssignedPermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
    expand: Option<Vec<GetAssignedPermissionSchemeRequestExpandValue>>,
}

impl<'a> GetAssignedPermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Note that permissions are included when you specify any value. Expand options include:
    ///
    ///  *  `all` Returns all expandable information.
    ///  *  `field` Returns information about the custom field granted the permission.
    ///  *  `group` Returns information about the group that is granted the permission.
    ///  *  `permissions` Returns all permission grants for each permission scheme.
    ///  *  `projectRole` Returns information about the project role granted the permission.
    ///  *  `user` Returns information about the user who is granted the permission.
    #[must_use]
    pub fn expand(
        mut self,
        value: impl IntoIterator<Item = impl Into<GetAssignedPermissionSchemeRequestExpandValue>>,
    ) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/project/{}/permissionscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push((
                "expand".to_owned(),
                crate::core::QueryValue::Scalar(value.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),
            ));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Assigns a permission scheme with a project. See [Managing project permissions](https://confluence.atlassian.com/x/yodKLg) for more information about permission schemes.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg)
#[derive(Clone)]
pub struct AssignPermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
    expand: Option<Vec<AssignPermissionSchemeRequestExpandValue>>,
    id: Id,
}

impl<'a> AssignPermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>, id: Id) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), id, expand: None }
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts a comma-separated list. Note that permissions are included when you specify any value. Expand options include:
    ///
    ///  *  `all` Returns all expandable information.
    ///  *  `field` Returns information about the custom field granted the permission.
    ///  *  `group` Returns information about the group that is granted the permission.
    ///  *  `permissions` Returns all permission grants for each permission scheme.
    ///  *  `projectRole` Returns information about the project role granted the permission.
    ///  *  `user` Returns information about the user who is granted the permission.
    #[must_use]
    pub fn expand(
        mut self,
        value: impl IntoIterator<Item = impl Into<AssignPermissionSchemeRequestExpandValue>>,
    ) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/project/{}/permissionscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push((
                "expand".to_owned(),
                crate::core::QueryValue::Scalar(value.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")),
            ));
        }

        let body = match serde_json::to_value(&self.id)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all [issue security](https://confluence.atlassian.com/x/J4lKLg) levels for the project that the user has access to.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project, however, issue security levels are only returned for authenticated user with *Set Issue Security* [global permission](https://confluence.atlassian.com/x/x4dKLg) for the project.
#[derive(Clone)]
pub struct GetSecurityLevelsForProjectRequest<'a> {
    client: &'a crate::core::Client,
    project_key_or_id: String,
}

impl<'a> GetSecurityLevelsForProjectRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/securitylevel", crate::core::encode_path_segment(&self.project_key_or_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectIssueSecurityLevels> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

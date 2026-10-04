// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::Serialize;

/// Use expand to include additional information about permission schemes in the response. This parameter accepts a comma-separated list of expandable options. Expand options include: all and field.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetAssignedPermissionSchemeRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(GetAssignedPermissionSchemeRequestExpand { One, Many, Other });

/// Use expand to include additional information about permission schemes in the response. This parameter accepts a comma-separated list of expandable options. Expand options include: all and field.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AssignPermissionSchemeRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(AssignPermissionSchemeRequestExpand { One, Many, Other });

/// The ProjectPermissionSchemes operations.
pub struct ProjectPermissionSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectPermissionSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the issue security scheme for project.
    pub fn get_project_issue_security_scheme(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetProjectIssueSecuritySchemeRequest<'a> {
        GetProjectIssueSecuritySchemeRequest::new(self.client, project_key_or_id)
    }

    /// Gets a permission scheme assigned with a project
    pub fn get_assigned_permission_scheme(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetAssignedPermissionSchemeRequest<'a> {
        GetAssignedPermissionSchemeRequest::new(self.client, project_key_or_id)
    }

    /// Assigns a permission scheme with a project
    pub fn assign_permission_scheme(
        &self,
        project_key_or_id: impl Into<String>,
        id: Id,
    ) -> AssignPermissionSchemeRequest<'a> {
        AssignPermissionSchemeRequest::new(self.client, project_key_or_id, id)
    }

    /// Returns all security levels for the project that the current logged in user has access to. If the user does not have the Set Issue Security permission, the list will be empty.
    pub fn get_security_levels_for_project(
        &self,
        project_key_or_id: impl Into<String>,
    ) -> GetSecurityLevelsForProjectRequest<'a> {
        GetSecurityLevelsForProjectRequest::new(self.client, project_key_or_id)
    }
}

/// Returns the issue security scheme for project.
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
                "/rest/api/2/project/{}/issuesecuritylevelscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SecuritySchemeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Gets a permission scheme assigned with a project
#[derive(Clone)]
pub struct GetAssignedPermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetAssignedPermissionSchemeRequestExpand>,
    project_key_or_id: String,
}

impl<'a> GetAssignedPermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), expand: None }
    }

    /// Use expand to include additional information about permission schemes in the response. This parameter accepts a comma-separated list of expandable options. Expand options include: all and field.
    #[must_use]
    pub fn expand(mut self, value: GetAssignedPermissionSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/project/{}/permissionscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Assigns a permission scheme with a project
#[derive(Clone)]
pub struct AssignPermissionSchemeRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<AssignPermissionSchemeRequestExpand>,
    project_key_or_id: String,
    id: Id,
}

impl<'a> AssignPermissionSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key_or_id: impl Into<String>, id: Id) -> Self {
        Self { client, project_key_or_id: project_key_or_id.into(), id, expand: None }
    }

    /// Use expand to include additional information about permission schemes in the response. This parameter accepts a comma-separated list of expandable options. Expand options include: all and field.
    #[must_use]
    pub fn expand(mut self, value: AssignPermissionSchemeRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/project/{}/permissionscheme",
                crate::core::encode_path_segment(&self.project_key_or_id)
            ),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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

/// Returns all security levels for the project that the current logged in user has access to. If the user does not have the Set Issue Security permission, the list will be empty.
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
            format!("/rest/api/2/project/{}/securitylevel", crate::core::encode_path_segment(&self.project_key_or_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SecurityListLevelJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

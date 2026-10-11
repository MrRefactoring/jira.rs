// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectRoles operations.
pub struct ProjectRolesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectRolesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all roles in the given project Id or key, with links to full details on each role.
    pub fn get_project_roles(&self, project_id_or_key: impl Into<String>) -> GetProjectRolesRequest<'a> {
        GetProjectRolesRequest::new(self.client, project_id_or_key)
    }

    /// Returns the details for a given project role in a project.
    pub fn get_project_role(&self, project_id_or_key: impl Into<String>, id: i64) -> GetProjectRoleRequest<'a> {
        GetProjectRoleRequest::new(self.client, project_id_or_key, id)
    }

    /// Get all the ProjectRoles available in Jira. Currently this list is global.
    pub fn get_all_project_roles(&self) -> GetAllProjectRolesRequest<'a> {
        GetAllProjectRolesRequest::new(self.client)
    }

    /// Creates a new ProjectRole to be available in Jira. The created role does not have any default actors assigned.
    pub fn create_project_role(
        &self,
        create_update_role_request: CreateUpdateRoleRequest,
    ) -> CreateProjectRoleRequest<'a> {
        CreateProjectRoleRequest::new(self.client, create_update_role_request)
    }

    /// Get a specific ProjectRole available in Jira.
    pub fn get_project_role_by_id(&self, id: i64) -> GetProjectRoleByIdRequest<'a> {
        GetProjectRoleByIdRequest::new(self.client, id)
    }

    /// Partially updates a roles name or description.
    pub fn partial_update_project_role(
        &self,
        id: i64,
        create_update_role_request: CreateUpdateRoleRequest,
    ) -> PartialUpdateProjectRoleRequest<'a> {
        PartialUpdateProjectRoleRequest::new(self.client, id, create_update_role_request)
    }

    /// Fully updates a roles. Both name and description must be given.
    pub fn fully_update_project_role(
        &self,
        id: i64,
        create_update_role_request: CreateUpdateRoleRequest,
    ) -> FullyUpdateProjectRoleRequest<'a> {
        FullyUpdateProjectRoleRequest::new(self.client, id, create_update_role_request)
    }

    /// Deletes a role. May return 403 in the future
    pub fn delete_project_role(&self, id: i64) -> DeleteProjectRoleRequest<'a> {
        DeleteProjectRoleRequest::new(self.client, id)
    }
}

/// Returns all roles in the given project Id or key, with links to full details on each role.
#[derive(Clone)]
pub struct GetProjectRolesRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
}

impl<'a> GetProjectRolesRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/{}/role", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetProjectRoles> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the details for a given project role in a project.
#[derive(Clone)]
pub struct GetProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    id: i64,
}

impl<'a> GetProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>, id: i64) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/project/{}/role/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                self.id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRole> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get all the ProjectRoles available in Jira. Currently this list is global.
#[derive(Clone)]
pub struct GetAllProjectRolesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllProjectRolesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/role".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ProjectRole>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a new ProjectRole to be available in Jira. The created role does not have any default actors assigned.
#[derive(Clone)]
pub struct CreateProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    create_update_role_request: CreateUpdateRoleRequest,
}

impl<'a> CreateProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, create_update_role_request: CreateUpdateRoleRequest) -> Self {
        Self { client, create_update_role_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/role".to_owned());

        let body = match serde_json::to_value(&self.create_update_role_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRole> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a specific ProjectRole available in Jira.
#[derive(Clone)]
pub struct GetProjectRoleByIdRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetProjectRoleByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/2/role/{}", self.id));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRole> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Partially updates a roles name or description.
#[derive(Clone)]
pub struct PartialUpdateProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    create_update_role_request: CreateUpdateRoleRequest,
}

impl<'a> PartialUpdateProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, create_update_role_request: CreateUpdateRoleRequest) -> Self {
        Self { client, id, create_update_role_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, format!("/rest/api/2/role/{}", self.id));

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.create_update_role_request)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRole> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Fully updates a roles. Both name and description must be given.
#[derive(Clone)]
pub struct FullyUpdateProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    create_update_role_request: CreateUpdateRoleRequest,
}

impl<'a> FullyUpdateProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, create_update_role_request: CreateUpdateRoleRequest) -> Self {
        Self { client, id, create_update_role_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/2/role/{}", self.id));

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.create_update_role_request)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRole> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a role. May return 403 in the future
#[derive(Clone)]
pub struct DeleteProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    swap: Option<i64>,
    id: i64,
}

impl<'a> DeleteProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, swap: None }
    }

    /// If given, removes a role even if it is used in scheme by replacing the role with the given one
    #[must_use]
    pub fn swap(mut self, value: i64) -> Self {
        self.swap = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, format!("/rest/api/2/role/{}", self.id));

        if let Some(value) = &self.swap {
            config.query.push(("swap".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

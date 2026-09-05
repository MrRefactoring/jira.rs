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
    pub fn get_project_roles_by_id(&self, id: i64) -> GetProjectRolesByIdRequest<'a> {
        GetProjectRolesByIdRequest::new(self.client, id)
    }

    /// Partially updates a roles name or description.
    pub fn partial_update_project_role(&self, id: i64) -> PartialUpdateProjectRoleRequest<'a> {
        PartialUpdateProjectRoleRequest::new(self.client, id)
    }

    /// Fully updates a roles. Both name and description must be given.
    pub fn fully_update_project_role(&self, id: i64) -> FullyUpdateProjectRoleRequest<'a> {
        FullyUpdateProjectRoleRequest::new(self.client, id)
    }

    /// Deletes a role. May return 403 in the future
    pub fn delete_project_role(&self, id: i64) -> DeleteProjectRoleRequest<'a> {
        DeleteProjectRoleRequest::new(self.client, id)
    }

    /// Gets default actors for the given role.
    pub fn get_project_role_actors_for_role(&self, id: i64) -> GetProjectRoleActorsForRoleRequest<'a> {
        GetProjectRoleActorsForRoleRequest::new(self.client, id)
    }

    /// Adds default actors to the given role. The request data should contain a list of usernames or a list of groups to add.
    pub fn add_project_role_actors_to_role(&self, id: i64) -> AddProjectRoleActorsToRoleRequest<'a> {
        AddProjectRoleActorsToRoleRequest::new(self.client, id)
    }

    /// Removes default actor from the given role.
    pub fn delete_project_role_actors_from_role(&self, id: i64) -> DeleteProjectRoleActorsFromRoleRequest<'a> {
        DeleteProjectRoleActorsFromRoleRequest::new(self.client, id)
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
pub struct GetProjectRolesByIdRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetProjectRolesByIdRequest<'a> {
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
    create_update_role_request: Option<CreateUpdateRoleRequest>,
}

impl<'a> PartialUpdateProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, create_update_role_request: None }
    }

    #[must_use]
    pub fn create_update_role_request(mut self, value: CreateUpdateRoleRequest) -> Self {
        self.create_update_role_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, format!("/rest/api/2/role/{}", self.id));

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

/// Fully updates a roles. Both name and description must be given.
#[derive(Clone)]
pub struct FullyUpdateProjectRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    create_update_role_request: Option<CreateUpdateRoleRequest>,
}

impl<'a> FullyUpdateProjectRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, create_update_role_request: None }
    }

    #[must_use]
    pub fn create_update_role_request(mut self, value: CreateUpdateRoleRequest) -> Self {
        self.create_update_role_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/2/role/{}", self.id));

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

/// Gets default actors for the given role.
#[derive(Clone)]
pub struct GetProjectRoleActorsForRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
}

impl<'a> GetProjectRoleActorsForRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/2/role/{}/actors", self.id));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRoleActors> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds default actors to the given role. The request data should contain a list of usernames or a list of groups to add.
#[derive(Clone)]
pub struct AddProjectRoleActorsToRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    actor_input: Option<ActorInput>,
}

impl<'a> AddProjectRoleActorsToRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, actor_input: None }
    }

    #[must_use]
    pub fn actor_input(mut self, value: ActorInput) -> Self {
        self.actor_input = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, format!("/rest/api/2/role/{}/actors", self.id));

        let body = match serde_json::to_value(&self.actor_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRoleActors> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes default actor from the given role.
#[derive(Clone)]
pub struct DeleteProjectRoleActorsFromRoleRequest<'a> {
    client: &'a crate::core::Client,
    id: i64,
    user: Option<String>,
    group: Option<String>,
}

impl<'a> DeleteProjectRoleActorsFromRoleRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, user: None, group: None }
    }

    /// If given, removes an actor from given role
    #[must_use]
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());

        self
    }

    /// If given, removes an actor from given role
    #[must_use]
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/role/{}/actors", self.id),
        );

        if let Some(value) = &self.user {
            config.query.push(("user".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group {
            config.query.push(("group".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectRoleActors> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

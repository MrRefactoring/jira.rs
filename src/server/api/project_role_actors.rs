// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectRoleActors operations.
pub struct ProjectRoleActorsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectRoleActorsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Adds an actor (user or group) to a project role. For user actors, their usernames should be used.
    pub fn add_actor_users(
        &self,
        project_id_or_key: impl Into<String>,
        id: i64,
        actors_map: ActorsMap,
    ) -> AddActorUsersRequest<'a> {
        AddActorUsersRequest::new(self.client, project_id_or_key, id, actors_map)
    }

    /// Updates a project role to include the specified actors (users or groups). Can be also used to clear roles to not include any users or groups. For user actors, their usernames should be used.
    pub fn set_actors(
        &self,
        project_id_or_key: impl Into<String>,
        id: i64,
        body: ProjectRoleActorsUpdate,
    ) -> SetActorsRequest<'a> {
        SetActorsRequest::new(self.client, project_id_or_key, id, body)
    }

    /// Deletes actors (users or groups) from a project role.
    pub fn delete_actor(&self, project_id_or_key: impl Into<String>, id: i64) -> DeleteActorRequest<'a> {
        DeleteActorRequest::new(self.client, project_id_or_key, id)
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

/// Adds an actor (user or group) to a project role. For user actors, their usernames should be used.
#[derive(Clone)]
pub struct AddActorUsersRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    id: i64,
    actors_map: ActorsMap,
}

impl<'a> AddActorUsersRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        id: i64,
        actors_map: ActorsMap,
    ) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), id, actors_map }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/api/2/project/{}/role/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                self.id
            ),
        );

        let body = match serde_json::to_value(&self.actors_map)? {
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

/// Updates a project role to include the specified actors (users or groups). Can be also used to clear roles to not include any users or groups. For user actors, their usernames should be used.
#[derive(Clone)]
pub struct SetActorsRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    id: i64,
    body: ProjectRoleActorsUpdate,
}

impl<'a> SetActorsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_id_or_key: impl Into<String>,
        id: i64,
        body: ProjectRoleActorsUpdate,
    ) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/project/{}/role/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                self.id
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Deletes actors (users or groups) from a project role.
#[derive(Clone)]
pub struct DeleteActorRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    id: i64,
    user: Option<String>,
    group: Option<String>,
}

impl<'a> DeleteActorRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>, id: i64) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), id, user: None, group: None }
    }

    /// The user name of the user to remove from the project role. Use either user or group, but not both
    #[must_use]
    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());

        self
    }

    /// The group name to remove from the project role. Use either user or group, but not both
    #[must_use]
    pub fn group(mut self, value: impl Into<String>) -> Self {
        self.group = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/project/{}/role/{}",
                crate::core::encode_path_segment(&self.project_id_or_key),
                self.id
            ),
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

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Permissions operations.
pub struct PermissionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PermissionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all permissions in the system and whether the currently logged in user has them. You can optionally provide a specific context to get permissions for (projectKey OR projectId OR issueKey OR issueId)
    pub fn get_permissions(&self) -> GetPermissionsRequest<'a> {
        GetPermissionsRequest::new(self.client)
    }

    /// Returns all permissions that are present in the Jira instance - Global, Project and the global ones added by plugins
    pub fn get_all_permissions(&self) -> GetAllPermissionsRequest<'a> {
        GetAllPermissionsRequest::new(self.client)
    }
}

/// Returns all permissions in the system and whether the currently logged in user has them. You can optionally provide a specific context to get permissions for (projectKey OR projectId OR issueKey OR issueId)
pub struct GetPermissionsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id: Option<String>,
    project_key: Option<String>,
    issue_key: Option<String>,
    project_id: Option<String>,
}

impl<'a> GetPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, issue_id: None, project_key: None, issue_key: None, project_id: None }
    }

    /// id of the issue to scope returned permissions for.
    #[must_use]
    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());

        self
    }

    /// key of project to scope returned permissions for.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// key of the issue to scope returned permissions for.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// id of project to scope returned permissions for.
    #[must_use]
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/mypermissions".to_owned());

        if let Some(value) = &self.issue_id {
            config.query.push(("issueId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionsJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all permissions that are present in the Jira instance - Global, Project and the global ones added by plugins
pub struct GetAllPermissionsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/permissions".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PermissionsJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectTypes operations.
pub struct ProjectTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all the project types defined on the Jira instance, not taking into account whether the license to use those project types is valid or not. In case of anonymous checks if they can access at least one project.
    pub fn get_all_project_types(&self) -> GetAllProjectTypesRequest<'a> {
        GetAllProjectTypesRequest::new(self.client)
    }

    /// Returns the project type with the given key. In case of anonymous checks if they can access at least one project.
    pub fn get_project_type_by_key(&self, project_type_key: impl Into<String>) -> GetProjectTypeByKeyRequest<'a> {
        GetProjectTypeByKeyRequest::new(self.client, project_type_key)
    }

    /// Returns the project type with the given key, if it is accessible to the logged in user. This takes into account whether the user is licensed on the Application that defines the project type.
    pub fn get_accessible_project_type_by_key(
        &self,
        project_type_key: impl Into<String>,
    ) -> GetAccessibleProjectTypeByKeyRequest<'a> {
        GetAccessibleProjectTypeByKeyRequest::new(self.client, project_type_key)
    }
}

/// Returns all the project types defined on the Jira instance, not taking into account whether the license to use those project types is valid or not. In case of anonymous checks if they can access at least one project.
#[derive(Clone)]
pub struct GetAllProjectTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllProjectTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/project/type".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ProjectType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the project type with the given key. In case of anonymous checks if they can access at least one project.
#[derive(Clone)]
pub struct GetProjectTypeByKeyRequest<'a> {
    client: &'a crate::core::Client,
    project_type_key: String,
}

impl<'a> GetProjectTypeByKeyRequest<'a> {
    fn new(client: &'a crate::core::Client, project_type_key: impl Into<String>) -> Self {
        Self { client, project_type_key: project_type_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/type/{}", crate::core::encode_path_segment(&self.project_type_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the project type with the given key, if it is accessible to the logged in user. This takes into account whether the user is licensed on the Application that defines the project type.
#[derive(Clone)]
pub struct GetAccessibleProjectTypeByKeyRequest<'a> {
    client: &'a crate::core::Client,
    project_type_key: String,
}

impl<'a> GetAccessibleProjectTypeByKeyRequest<'a> {
    fn new(client: &'a crate::core::Client, project_type_key: impl Into<String>) -> Self {
        Self { client, project_type_key: project_type_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/project/type/{}/accessible", crate::core::encode_path_segment(&self.project_type_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

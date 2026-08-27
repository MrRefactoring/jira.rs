// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// The key of the project type.
    pub enum GetProjectTypeByKeyRequestProjectTypeKey {
        Software => "software",
        ServiceDesk => "service_desk",
        Business => "business",
        ProductDiscovery => "product_discovery",
    }
}

crate::open_enum! {
    /// The key of the project type.
    pub enum GetAccessibleProjectTypeByKeyRequestProjectTypeKey {
        Software => "software",
        ServiceDesk => "service_desk",
        Business => "business",
        ProductDiscovery => "product_discovery",
    }
}

/// The ProjectTypes operations.
pub struct ProjectTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all [project types](https://confluence.atlassian.com/x/Var1Nw), whether or not the instance has a valid license for each type.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn get_all_project_types(&self) -> GetAllProjectTypesRequest<'a> {
        GetAllProjectTypesRequest::new(self.client)
    }

    /// Returns all [project types](https://confluence.atlassian.com/x/Var1Nw) with a valid license.
    pub fn get_all_accessible_project_types(&self) -> GetAllAccessibleProjectTypesRequest<'a> {
        GetAllAccessibleProjectTypesRequest::new(self.client)
    }

    /// Returns a [project type](https://confluence.atlassian.com/x/Var1Nw).
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn get_project_type_by_key(
        &self,
        project_type_key: impl Into<GetProjectTypeByKeyRequestProjectTypeKey>,
    ) -> GetProjectTypeByKeyRequest<'a> {
        GetProjectTypeByKeyRequest::new(self.client, project_type_key)
    }

    /// Returns a [project type](https://confluence.atlassian.com/x/Var1Nw) if it is accessible to the user.
    ///
    /// **[Permissions](#permissions) required:** Permission to access Jira.
    pub fn get_accessible_project_type_by_key(
        &self,
        project_type_key: impl Into<GetAccessibleProjectTypeByKeyRequestProjectTypeKey>,
    ) -> GetAccessibleProjectTypeByKeyRequest<'a> {
        GetAccessibleProjectTypeByKeyRequest::new(self.client, project_type_key)
    }
}

/// Returns all [project types](https://confluence.atlassian.com/x/Var1Nw), whether or not the instance has a valid license for each type.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None.
pub struct GetAllProjectTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllProjectTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/project/type".to_owned());

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

/// Returns all [project types](https://confluence.atlassian.com/x/Var1Nw) with a valid license.
pub struct GetAllAccessibleProjectTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllAccessibleProjectTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/project/type/accessible".to_owned());

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

/// Returns a [project type](https://confluence.atlassian.com/x/Var1Nw).
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None.
pub struct GetProjectTypeByKeyRequest<'a> {
    client: &'a crate::core::Client,
    project_type_key: GetProjectTypeByKeyRequestProjectTypeKey,
}

impl<'a> GetProjectTypeByKeyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_type_key: impl Into<GetProjectTypeByKeyRequestProjectTypeKey>,
    ) -> Self {
        Self { client, project_type_key: project_type_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/type/{}", self.project_type_key),
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

/// Returns a [project type](https://confluence.atlassian.com/x/Var1Nw) if it is accessible to the user.
///
/// **[Permissions](#permissions) required:** Permission to access Jira.
pub struct GetAccessibleProjectTypeByKeyRequest<'a> {
    client: &'a crate::core::Client,
    project_type_key: GetAccessibleProjectTypeByKeyRequestProjectTypeKey,
}

impl<'a> GetAccessibleProjectTypeByKeyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        project_type_key: impl Into<GetAccessibleProjectTypeByKeyRequestProjectTypeKey>,
    ) -> Self {
        Self { client, project_type_key: project_type_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/type/{}/accessible", self.project_type_key),
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

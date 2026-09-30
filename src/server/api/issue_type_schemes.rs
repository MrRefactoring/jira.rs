// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueTypeSchemes operations.
pub struct IssueTypeSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueTypeSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all issue type schemes visible to the user. All issue types associated with the scheme will only be returned if an additional query parameter is provided: expand=schemes.issueTypes. Similarly, the default issue type associated with the scheme (if one exists) will only be returned if an additional query parameter is provided: expand=schemes.defaultIssueType. Note that both query parameters can be used together: expand=schemes.issueTypes,schemes.defaultIssueType.
    pub fn get_all_issue_type_schemes(&self) -> GetAllIssueTypeSchemesRequest<'a> {
        GetAllIssueTypeSchemesRequest::new(self.client)
    }

    /// Creates an issue type scheme from a JSON representation
    pub fn create_issue_type_scheme(
        &self,
        issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate,
    ) -> CreateIssueTypeSchemeRequest<'a> {
        CreateIssueTypeSchemeRequest::new(self.client, issue_type_scheme_create_update)
    }

    /// Returns a full representation of the issue type scheme that has the given id
    pub fn get_issue_type_scheme(&self, scheme_id: impl Into<String>) -> GetIssueTypeSchemeRequest<'a> {
        GetIssueTypeSchemeRequest::new(self.client, scheme_id)
    }

    /// Updates the specified issue type scheme from a JSON representation
    pub fn update_issue_type_scheme(
        &self,
        scheme_id: impl Into<String>,
        issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate,
    ) -> UpdateIssueTypeSchemeRequest<'a> {
        UpdateIssueTypeSchemeRequest::new(self.client, scheme_id, issue_type_scheme_create_update)
    }

    /// Deletes the specified issue type scheme. Any projects associated with this IssueTypeScheme will be automatically associated with the global default IssueTypeScheme.
    pub fn delete_issue_type_scheme(&self, scheme_id: impl Into<String>) -> DeleteIssueTypeSchemeRequest<'a> {
        DeleteIssueTypeSchemeRequest::new(self.client, scheme_id)
    }

    /// For the specified issue type scheme, returns all of the associated projects
    pub fn get_associated_projects(&self, scheme_id: impl Into<String>) -> GetAssociatedProjectsRequest<'a> {
        GetAssociatedProjectsRequest::new(self.client, scheme_id)
    }

    /// Adds additional projects to those already associated with the specified issue type scheme
    pub fn add_project_associations_to_scheme(
        &self,
        scheme_id: impl Into<String>,
        associate_projects: AssociateProjects,
    ) -> AddProjectAssociationsToSchemeRequest<'a> {
        AddProjectAssociationsToSchemeRequest::new(self.client, scheme_id, associate_projects)
    }

    /// Associates the given projects with the specified issue type scheme
    pub fn set_project_associations_for_scheme(
        &self,
        scheme_id: impl Into<String>,
        associate_projects: AssociateProjects,
    ) -> SetProjectAssociationsForSchemeRequest<'a> {
        SetProjectAssociationsForSchemeRequest::new(self.client, scheme_id, associate_projects)
    }

    /// Removes all project associations for the specified issue type scheme
    pub fn remove_all_project_associations(
        &self,
        scheme_id: impl Into<String>,
    ) -> RemoveAllProjectAssociationsRequest<'a> {
        RemoveAllProjectAssociationsRequest::new(self.client, scheme_id)
    }

    /// For the specified issue type scheme, removes the given project association
    pub fn remove_project_association(
        &self,
        proj_id_or_key: impl Into<String>,
        scheme_id: impl Into<String>,
    ) -> RemoveProjectAssociationRequest<'a> {
        RemoveProjectAssociationRequest::new(self.client, proj_id_or_key, scheme_id)
    }
}

/// Returns a list of all issue type schemes visible to the user. All issue types associated with the scheme will only be returned if an additional query parameter is provided: expand=schemes.issueTypes. Similarly, the default issue type associated with the scheme (if one exists) will only be returned if an additional query parameter is provided: expand=schemes.defaultIssueType. Note that both query parameters can be used together: expand=schemes.issueTypes,schemes.defaultIssueType.
#[derive(Clone)]
pub struct GetAllIssueTypeSchemesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAllIssueTypeSchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/issuetypescheme".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeSchemeList> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates an issue type scheme from a JSON representation
#[derive(Clone)]
pub struct CreateIssueTypeSchemeRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate,
}

impl<'a> CreateIssueTypeSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate) -> Self {
        Self { client, issue_type_scheme_create_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issuetypescheme".to_owned());

        let body = match serde_json::to_value(&self.issue_type_scheme_create_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of the issue type scheme that has the given id
#[derive(Clone)]
pub struct GetIssueTypeSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
}

impl<'a> GetIssueTypeSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: impl Into<String>) -> Self {
        Self { client, scheme_id: scheme_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issuetypescheme/{}", crate::core::encode_path_segment(&self.scheme_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates the specified issue type scheme from a JSON representation
#[derive(Clone)]
pub struct UpdateIssueTypeSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
    issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate,
}

impl<'a> UpdateIssueTypeSchemeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        scheme_id: impl Into<String>,
        issue_type_scheme_create_update: IssueTypeSchemeCreateUpdate,
    ) -> Self {
        Self { client, scheme_id: scheme_id.into(), issue_type_scheme_create_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issuetypescheme/{}", crate::core::encode_path_segment(&self.scheme_id)),
        );

        let body = match serde_json::to_value(&self.issue_type_scheme_create_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueTypeScheme> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes the specified issue type scheme. Any projects associated with this IssueTypeScheme will be automatically associated with the global default IssueTypeScheme.
#[derive(Clone)]
pub struct DeleteIssueTypeSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
}

impl<'a> DeleteIssueTypeSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: impl Into<String>) -> Self {
        Self { client, scheme_id: scheme_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issuetypescheme/{}", crate::core::encode_path_segment(&self.scheme_id)),
        );

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

/// For the specified issue type scheme, returns all of the associated projects
#[derive(Clone)]
pub struct GetAssociatedProjectsRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<String>,
    scheme_id: String,
}

impl<'a> GetAssociatedProjectsRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: impl Into<String>) -> Self {
        Self { client, scheme_id: scheme_id.into(), expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issuetypescheme/{}/associations", crate::core::encode_path_segment(&self.scheme_id)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Project>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds additional projects to those already associated with the specified issue type scheme
#[derive(Clone)]
pub struct AddProjectAssociationsToSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
    associate_projects: AssociateProjects,
}

impl<'a> AddProjectAssociationsToSchemeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        scheme_id: impl Into<String>,
        associate_projects: AssociateProjects,
    ) -> Self {
        Self { client, scheme_id: scheme_id.into(), associate_projects }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issuetypescheme/{}/associations", crate::core::encode_path_segment(&self.scheme_id)),
        );

        let body = match serde_json::to_value(&self.associate_projects)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Associates the given projects with the specified issue type scheme
#[derive(Clone)]
pub struct SetProjectAssociationsForSchemeRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
    associate_projects: AssociateProjects,
}

impl<'a> SetProjectAssociationsForSchemeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        scheme_id: impl Into<String>,
        associate_projects: AssociateProjects,
    ) -> Self {
        Self { client, scheme_id: scheme_id.into(), associate_projects }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/issuetypescheme/{}/associations", crate::core::encode_path_segment(&self.scheme_id)),
        );

        let body = match serde_json::to_value(&self.associate_projects)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Removes all project associations for the specified issue type scheme
#[derive(Clone)]
pub struct RemoveAllProjectAssociationsRequest<'a> {
    client: &'a crate::core::Client,
    scheme_id: String,
}

impl<'a> RemoveAllProjectAssociationsRequest<'a> {
    fn new(client: &'a crate::core::Client, scheme_id: impl Into<String>) -> Self {
        Self { client, scheme_id: scheme_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issuetypescheme/{}/associations", crate::core::encode_path_segment(&self.scheme_id)),
        );

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

/// For the specified issue type scheme, removes the given project association
#[derive(Clone)]
pub struct RemoveProjectAssociationRequest<'a> {
    client: &'a crate::core::Client,
    proj_id_or_key: String,
    scheme_id: String,
}

impl<'a> RemoveProjectAssociationRequest<'a> {
    fn new(client: &'a crate::core::Client, proj_id_or_key: impl Into<String>, scheme_id: impl Into<String>) -> Self {
        Self { client, proj_id_or_key: proj_id_or_key.into(), scheme_id: scheme_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/issuetypescheme/{}/associations/{}",
                crate::core::encode_path_segment(&self.scheme_id),
                crate::core::encode_path_segment(&self.proj_id_or_key)
            ),
        );

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

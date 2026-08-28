// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by the component description.
    ///  *  `name` Sorts by component name.
    pub enum FindComponentsForProjectsRequestOrderBy {
        Description => "description",
        DescriptionDescending => "-description",
        DescriptionAscending => "+description",
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
    }
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by the component description.
    ///  *  `issueCount` Sorts by the count of issues associated with the component.
    ///  *  `lead` Sorts by the user key of the component's project lead.
    ///  *  `name` Sorts by component name.
    pub enum GetProjectComponentsPaginatedRequestOrderBy {
        Description => "description",
        DescriptionDescending => "-description",
        DescriptionAscending => "+description",
        IssueCount => "issueCount",
        IssueCountDescending => "-issueCount",
        IssueCountAscending => "+issueCount",
        Lead => "lead",
        LeadDescending => "-lead",
        LeadAscending => "+lead",
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
    }
}

crate::open_enum! {
    /// The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`.
    pub enum GetProjectComponentsPaginatedRequestComponentSource {
        Jira => "jira",
        Compass => "compass",
        Auto => "auto",
    }
}

crate::open_enum! {
    /// The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`.
    pub enum GetProjectComponentsRequestComponentSource {
        Jira => "jira",
        Compass => "compass",
        Auto => "auto",
    }
}

/// The ProjectComponents operations.
pub struct ProjectComponentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectComponentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all components in a project, including global (Compass) components when applicable.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn find_components_for_projects(&self) -> FindComponentsForProjectsRequest<'a> {
        FindComponentsForProjectsRequest::new(self.client)
    }

    /// Creates a component. Use components to provide containers for issues within a project. Use components to provide containers for issues within a project.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project in which the component is created or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_component(&self, project_component: ProjectComponent) -> CreateComponentRequest<'a> {
        CreateComponentRequest::new(self.client, project_component)
    }

    /// Returns a component.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for project containing the component.
    pub fn get_component(&self, id: impl Into<String>) -> GetComponentRequest<'a> {
        GetComponentRequest::new(self.client, id)
    }

    /// Updates a component. Any fields included in the request are overwritten. If `leadAccountId` is an empty string ("") the component lead is removed.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn update_component(&self, id: impl Into<String>, body: ProjectComponent) -> UpdateComponentRequest<'a> {
        UpdateComponentRequest::new(self.client, id, body)
    }

    /// Deletes a component.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_component(&self, id: impl Into<String>) -> DeleteComponentRequest<'a> {
        DeleteComponentRequest::new(self.client, id)
    }

    /// Returns the counts of issues assigned to the component.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
    ///
    ///  *  **Classic**: `read:jira-work`
    ///  *  **Granular**: `read:field:jira`, `read:project.component:jira`
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_component_related_issues(&self, id: impl Into<String>) -> GetComponentRelatedIssuesRequest<'a> {
        GetComponentRelatedIssuesRequest::new(self.client, id)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all components in a project. See the [Get project components](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-components-get) resource if you want to get a full list of versions without pagination.
    ///
    /// If your project uses Compass components, this API will return a list of Compass components that are linked to issues in that project.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_project_components_paginated(
        &self,
        project_id_or_key: impl Into<String>,
    ) -> GetProjectComponentsPaginatedRequest<'a> {
        GetProjectComponentsPaginatedRequest::new(self.client, project_id_or_key)
    }

    /// Returns all components in a project. See the [Get project components paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-component-get) resource if you want to get a full list of components with pagination.
    ///
    /// If your project uses Compass components, this API will return a paginated list of Compass components that are linked to issues in that project.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_project_components(&self, project_id_or_key: impl Into<String>) -> GetProjectComponentsRequest<'a> {
        GetProjectComponentsRequest::new(self.client, project_id_or_key)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all components in a project, including global (Compass) components when applicable.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
#[derive(Clone)]
pub struct FindComponentsForProjectsRequest<'a> {
    client: &'a crate::core::Client,
    project_ids_or_keys: Option<Vec<String>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    order_by: Option<FindComponentsForProjectsRequestOrderBy>,
    query: Option<String>,
}

impl<'a> FindComponentsForProjectsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, project_ids_or_keys: None, start_at: None, max_results: None, order_by: None, query: None }
    }

    /// The project IDs and/or project keys (case sensitive).
    #[must_use]
    pub fn project_ids_or_keys(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.project_ids_or_keys = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by the component description.
    ///  *  `name` Sorts by component name.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<FindComponentsForProjectsRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// Filter the results using a literal string. Components with a matching `name` or `description` are returned (case insensitive).
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/component".to_owned());

        if let Some(value) = &self.project_ids_or_keys {
            config.query.push(("projectIdsOrKeys".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Component>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a component. Use components to provide containers for issues within a project. Use components to provide containers for issues within a project.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project in which the component is created or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct CreateComponentRequest<'a> {
    client: &'a crate::core::Client,
    project_component: ProjectComponent,
}

impl<'a> CreateComponentRequest<'a> {
    fn new(client: &'a crate::core::Client, project_component: ProjectComponent) -> Self {
        Self { client, project_component }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/component".to_owned());

        let body = match serde_json::to_value(&self.project_component)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectComponent> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a component.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for project containing the component.
#[derive(Clone)]
pub struct GetComponentRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetComponentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/component/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectComponent> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a component. Any fields included in the request are overwritten. If `leadAccountId` is an empty string ("") the component lead is removed.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct UpdateComponentRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: ProjectComponent,
}

impl<'a> UpdateComponentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, body: ProjectComponent) -> Self {
        Self { client, id: id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/component/{}", crate::core::encode_path_segment(&self.id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectComponent> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a component.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the component or *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct DeleteComponentRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    move_issues_to: Option<String>,
}

impl<'a> DeleteComponentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), move_issues_to: None }
    }

    /// The ID of the component to replace the deleted component. If this value is null no replacement is made.
    #[must_use]
    pub fn move_issues_to(mut self, value: impl Into<String>) -> Self {
        self.move_issues_to = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/component/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.move_issues_to {
            config.query.push(("moveIssuesTo".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Returns the counts of issues assigned to the component.
///
/// This operation can be accessed anonymously.
///
/// **Deprecation notice:** The required OAuth 2.0 scopes will be updated on June 15, 2024.
///
///  *  **Classic**: `read:jira-work`
///  *  **Granular**: `read:field:jira`, `read:project.component:jira`
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetComponentRelatedIssuesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetComponentRelatedIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/component/{}/relatedIssueCounts", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ComponentIssuesCount> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all components in a project. See the [Get project components](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-components-get) resource if you want to get a full list of versions without pagination.
///
/// If your project uses Compass components, this API will return a list of Compass components that are linked to issues in that project.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
#[derive(Clone)]
pub struct GetProjectComponentsPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
    order_by: Option<GetProjectComponentsPaginatedRequestOrderBy>,
    component_source: Option<GetProjectComponentsPaginatedRequestComponentSource>,
    query: Option<String>,
}

impl<'a> GetProjectComponentsPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            project_id_or_key: project_id_or_key.into(),
            start_at: None,
            max_results: None,
            order_by: None,
            component_source: None,
            query: None,
        }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `description` Sorts by the component description.
    ///  *  `issueCount` Sorts by the count of issues associated with the component.
    ///  *  `lead` Sorts by the user key of the component's project lead.
    ///  *  `name` Sorts by component name.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetProjectComponentsPaginatedRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`.
    #[must_use]
    pub fn component_source(mut self, value: impl Into<GetProjectComponentsPaginatedRequestComponentSource>) -> Self {
        self.component_source = Some(value.into());

        self
    }

    /// Filter the results using a literal string. Components with a matching `name` or `description` are returned (case insensitive).
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/component", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.component_source {
            config.query.push(("componentSource".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ComponentWithIssueCount>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all components in a project. See the [Get project components paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-project/#api-rest-api-3-project-projectIdOrKey-component-get) resource if you want to get a full list of components with pagination.
///
/// If your project uses Compass components, this API will return a paginated list of Compass components that are linked to issues in that project.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
#[derive(Clone)]
pub struct GetProjectComponentsRequest<'a> {
    client: &'a crate::core::Client,
    project_id_or_key: String,
    component_source: Option<GetProjectComponentsRequestComponentSource>,
}

impl<'a> GetProjectComponentsRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id_or_key: impl Into<String>) -> Self {
        Self { client, project_id_or_key: project_id_or_key.into(), component_source: None }
    }

    /// The source of the components to return. Can be `jira` (default), `compass` or `auto`. When `auto` is specified, the API will return connected Compass components if the project is opted into Compass, otherwise it will return Jira components. Defaults to `jira`.
    #[must_use]
    pub fn component_source(mut self, value: impl Into<GetProjectComponentsRequestComponentSource>) -> Self {
        self.component_source = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/components", crate::core::encode_path_segment(&self.project_id_or_key)),
        );

        if let Some(value) = &self.component_source {
            config.query.push(("componentSource".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ProjectComponent>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

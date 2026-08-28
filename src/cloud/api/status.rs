// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Status operations.
pub struct StatusService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> StatusService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of the statuses specified by one or more status IDs.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn get_statuses_by_id(&self, id: impl IntoIterator<Item = impl Into<String>>) -> GetStatusesByIdRequest<'a> {
        GetStatusesByIdRequest::new(self.client, id)
    }

    /// Creates statuses for a global or project scope.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn create_statuses(&self, status_create_request: StatusCreateRequest) -> CreateStatusesRequest<'a> {
        CreateStatusesRequest::new(self.client, status_create_request)
    }

    /// Updates statuses by ID.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn update_statuses(&self, status_update_request: StatusUpdateRequest) -> UpdateStatusesRequest<'a> {
        UpdateStatusesRequest::new(self.client, status_update_request)
    }

    /// Deletes statuses by ID.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn delete_statuses_by_id(
        &self,
        id: impl IntoIterator<Item = impl Into<String>>,
    ) -> DeleteStatusesByIdRequest<'a> {
        DeleteStatusesByIdRequest::new(self.client, id)
    }

    /// Returns a list of the statuses specified by one or more status names.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Browse projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn get_statuses_by_name(
        &self,
        name: impl IntoIterator<Item = impl Into<String>>,
    ) -> GetStatusesByNameRequest<'a> {
        GetStatusesByNameRequest::new(self.client, name)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro/#pagination) list of statuses that match a search on name or project.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    ///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn search(&self) -> SearchRequest2<'a> {
        SearchRequest2::new(self.client)
    }

    /// Returns a page of issue types in a project using a given status.
    pub fn get_project_issue_type_usages_for_status(
        &self,
        status_id: impl Into<String>,
        project_id: impl Into<String>,
    ) -> GetProjectIssueTypeUsagesForStatusRequest<'a> {
        GetProjectIssueTypeUsagesForStatusRequest::new(self.client, status_id, project_id)
    }

    /// Returns a page of projects using a given status.
    pub fn get_project_usages_for_status(&self, status_id: impl Into<String>) -> GetProjectUsagesForStatusRequest<'a> {
        GetProjectUsagesForStatusRequest::new(self.client, status_id)
    }

    /// Returns a page of workflows using a given status.
    pub fn get_workflow_usages_for_status(
        &self,
        status_id: impl Into<String>,
    ) -> GetWorkflowUsagesForStatusRequest<'a> {
        GetWorkflowUsagesForStatusRequest::new(self.client, status_id)
    }
}

/// Returns a list of the statuses specified by one or more status IDs.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct GetStatusesByIdRequest<'a> {
    client: &'a crate::core::Client,
    id: Vec<String>,
}

impl<'a> GetStatusesByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { client, id: id.into_iter().map(Into::into).collect() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/statuses".to_owned());

        config.query.push(("id".to_owned(), crate::core::QueryValue::List(self.id.clone())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<JiraStatus>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates statuses for a global or project scope.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct CreateStatusesRequest<'a> {
    client: &'a crate::core::Client,
    status_create_request: StatusCreateRequest,
}

impl<'a> CreateStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client, status_create_request: StatusCreateRequest) -> Self {
        Self { client, status_create_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/statuses".to_owned());

        let body = match serde_json::to_value(&self.status_create_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<JiraStatus>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates statuses by ID.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct UpdateStatusesRequest<'a> {
    client: &'a crate::core::Client,
    status_update_request: StatusUpdateRequest,
}

impl<'a> UpdateStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client, status_update_request: StatusUpdateRequest) -> Self {
        Self { client, status_update_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/statuses".to_owned());

        let body = match serde_json::to_value(&self.status_update_request)? {
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

/// Deletes statuses by ID.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct DeleteStatusesByIdRequest<'a> {
    client: &'a crate::core::Client,
    id: Vec<String>,
}

impl<'a> DeleteStatusesByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { client, id: id.into_iter().map(Into::into).collect() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/statuses".to_owned());

        config.query.push(("id".to_owned(), crate::core::QueryValue::List(self.id.clone())));

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

/// Returns a list of the statuses specified by one or more status names.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Browse projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct GetStatusesByNameRequest<'a> {
    client: &'a crate::core::Client,
    name: Vec<String>,
    project_id: Option<String>,
}

impl<'a> GetStatusesByNameRequest<'a> {
    fn new(client: &'a crate::core::Client, name: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { client, name: name.into_iter().map(Into::into).collect(), project_id: None }
    }

    /// The project the status is part of or null for global statuses.
    #[must_use]
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/statuses/byNames".to_owned());

        config.query.push(("name".to_owned(), crate::core::QueryValue::List(self.name.clone())));

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<JiraStatus>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro/#pagination) list of statuses that match a search on name or project.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
///  *  *Administer Jira* [project permission.](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct SearchRequest2<'a> {
    client: &'a crate::core::Client,
    project_id: Option<String>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    search_string: Option<String>,
    status_category: Option<String>,
    include_global_statuses: Option<bool>,
}

impl<'a> SearchRequest2<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            project_id: None,
            start_at: None,
            max_results: None,
            search_string: None,
            status_category: None,
            include_global_statuses: None,
        }
    }

    /// The project the status is part of or null for global statuses.
    #[must_use]
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());

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

    /// Term to match status names against or null to search for all statuses in the search scope.
    #[must_use]
    pub fn search_string(mut self, value: impl Into<String>) -> Self {
        self.search_string = Some(value.into());

        self
    }

    /// Category of the status to filter by. The supported values are: `TODO`, `IN_PROGRESS`, and `DONE`.
    #[must_use]
    pub fn status_category(mut self, value: impl Into<String>) -> Self {
        self.status_category = Some(value.into());

        self
    }

    /// Whether to include global statuses (scope = null, not tied to any project) in the response. Defaults to false. Only relevant for project scoped queries.
    #[must_use]
    pub fn include_global_statuses(mut self, value: bool) -> Self {
        self.include_global_statuses = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/statuses/search".to_owned());

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.search_string {
            config.query.push(("searchString".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.status_category {
            config.query.push(("statusCategory".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_global_statuses {
            config.query.push(("includeGlobalStatuses".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageOfStatuses> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a page of issue types in a project using a given status.
#[derive(Clone)]
pub struct GetProjectIssueTypeUsagesForStatusRequest<'a> {
    client: &'a crate::core::Client,
    status_id: String,
    project_id: String,
    next_page_token: Option<String>,
    max_results: Option<i64>,
}

impl<'a> GetProjectIssueTypeUsagesForStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, status_id: impl Into<String>, project_id: impl Into<String>) -> Self {
        Self {
            client,
            status_id: status_id.into(),
            project_id: project_id.into(),
            next_page_token: None,
            max_results: None,
        }
    }

    /// The cursor for pagination
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of results to return. Must be an integer between 1 and 200.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/statuses/{}/project/{}/issueTypeUsages",
                crate::core::encode_path_segment(&self.status_id),
                crate::core::encode_path_segment(&self.project_id)
            ),
        );

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusProjectIssueTypeUsageDTO> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a page of projects using a given status.
#[derive(Clone)]
pub struct GetProjectUsagesForStatusRequest<'a> {
    client: &'a crate::core::Client,
    status_id: String,
    next_page_token: Option<String>,
    max_results: Option<i64>,
}

impl<'a> GetProjectUsagesForStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, status_id: impl Into<String>) -> Self {
        Self { client, status_id: status_id.into(), next_page_token: None, max_results: None }
    }

    /// The cursor for pagination
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of results to return. Must be an integer between 1 and 200.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/statuses/{}/projectUsages", crate::core::encode_path_segment(&self.status_id)),
        );

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusProjectUsageDTO> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a page of workflows using a given status.
#[derive(Clone)]
pub struct GetWorkflowUsagesForStatusRequest<'a> {
    client: &'a crate::core::Client,
    status_id: String,
    next_page_token: Option<String>,
    max_results: Option<i64>,
}

impl<'a> GetWorkflowUsagesForStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, status_id: impl Into<String>) -> Self {
        Self { client, status_id: status_id.into(), next_page_token: None, max_results: None }
    }

    /// The cursor for pagination
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of results to return. Must be an integer between 1 and 200.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/statuses/{}/workflowUsages", crate::core::encode_path_segment(&self.status_id)),
        );

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusWorkflowUsageDTO> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

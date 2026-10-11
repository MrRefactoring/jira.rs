// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueSearch operations.
pub struct IssueSearchService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueSearchService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get issue picker resource
    pub fn get_issue_picker_resource(&self) -> GetIssuePickerResourceRequest<'a> {
        GetIssuePickerResourceRequest::new(self.client)
    }

    /// Searches for issues using JQL.
    /// Sorting
    /// the jql parameter is a full [JQL](http://confluence.atlassian.com/display/JIRA/Advanced+Searching)
    /// expression, and includes an ORDER BY clause.
    /// The fields param (which can be specified multiple times) gives a comma-separated list of fields
    /// to include in the response. This can be used to retrieve a subset of fields.
    /// A particular field can be excluded by prefixing it with a minus.
    /// By default, only navigable (*navigable) fields are returned in this search resource. Note: the default is different
    /// in the get-issue resource -- the default there all fields (*all).
    /// *all - include all fields
    /// *navigable - include just navigable fields
    /// summary,comment - include just the summary and comments
    /// -description - include navigable fields except the description (the default is *navigable for search)
    /// *all,-comment - include everything except comments
    /// GET vs POST:
    /// If the JQL query is too large to be encoded as a query param you should instead
    /// POST to this resource.
    /// Expanding Issues in the Search Result:
    /// It is possible to expand the issues returned by directly specifying the expansion on the expand parameter passed
    /// in to this resources.
    /// For instance, to expand the changelog for all the issues on the search result, it is necessary to
    /// specify changelog as one of the values to expand.
    pub fn search_issues(&self) -> SearchIssuesRequest<'a> {
        SearchIssuesRequest::new(self.client)
    }

    /// Performs a search using JQL.
    pub fn search_issues_post(&self, search_and_reconcile_request: SearchRequest) -> SearchIssuesPostRequest<'a> {
        SearchIssuesPostRequest::new(self.client, search_and_reconcile_request)
    }

    /// Available since Jira Data Center 11.3.
    pub fn get_error(&self) -> GetErrorRequest<'a> {
        GetErrorRequest::new(self.client)
    }
}

/// Get issue picker resource
#[derive(Clone)]
pub struct GetIssuePickerResourceRequest<'a> {
    client: &'a crate::core::Client,
    current_project_id: Option<String>,
    query: Option<String>,
    current_issue_key: Option<String>,
    show_sub_tasks: Option<bool>,
    current_jql: Option<String>,
    show_sub_task_parent: Option<bool>,
}

impl<'a> GetIssuePickerResourceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            current_project_id: None,
            query: None,
            current_issue_key: None,
            show_sub_tasks: None,
            current_jql: None,
            show_sub_task_parent: None,
        }
    }

    /// the id of the project in context of which the request is executed
    #[must_use]
    pub fn current_project_id(mut self, value: impl Into<String>) -> Self {
        self.current_project_id = Some(value.into());

        self
    }

    /// the query
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// the key of the issue in context of which the request is executed
    #[must_use]
    pub fn current_issue_key(mut self, value: impl Into<String>) -> Self {
        self.current_issue_key = Some(value.into());

        self
    }

    /// if set to false, subtasks will not be included in the list
    #[must_use]
    pub fn show_sub_tasks(mut self, value: bool) -> Self {
        self.show_sub_tasks = Some(value);

        self
    }

    /// the JQL in context of which the request is executed
    #[must_use]
    pub fn current_jql(mut self, value: impl Into<String>) -> Self {
        self.current_jql = Some(value.into());

        self
    }

    /// if set to false and request is executed in context of a subtask, the parent issue will not be included in the auto-completion result, even if it matches the query
    #[must_use]
    pub fn show_sub_task_parent(mut self, value: bool) -> Self {
        self.show_sub_task_parent = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/issue/picker".to_owned());

        if let Some(value) = &self.current_project_id {
            config.query.push(("currentProjectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.current_issue_key {
            config.query.push(("currentIssueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.show_sub_tasks {
            config.query.push(("showSubTasks".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.current_jql {
            config.query.push(("currentJQL".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.show_sub_task_parent {
            config.query.push(("showSubTaskParent".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssuePickerSuggestions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Searches for issues using JQL.
/// Sorting
/// the jql parameter is a full [JQL](http://confluence.atlassian.com/display/JIRA/Advanced+Searching)
/// expression, and includes an ORDER BY clause.
/// The fields param (which can be specified multiple times) gives a comma-separated list of fields
/// to include in the response. This can be used to retrieve a subset of fields.
/// A particular field can be excluded by prefixing it with a minus.
/// By default, only navigable (*navigable) fields are returned in this search resource. Note: the default is different
/// in the get-issue resource -- the default there all fields (*all).
/// *all - include all fields
/// *navigable - include just navigable fields
/// summary,comment - include just the summary and comments
/// -description - include navigable fields except the description (the default is *navigable for search)
/// *all,-comment - include everything except comments
/// GET vs POST:
/// If the JQL query is too large to be encoded as a query param you should instead
/// POST to this resource.
/// Expanding Issues in the Search Result:
/// It is possible to expand the issues returned by directly specifying the expansion on the expand parameter passed
/// in to this resources.
/// For instance, to expand the changelog for all the issues on the search result, it is necessary to
/// specify changelog as one of the values to expand.
#[derive(Clone)]
pub struct SearchIssuesRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    fields: Option<Vec<String>>,
    start_at: Option<i64>,
}

impl<'a> SearchIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, jql: None, max_results: None, validate_query: None, fields: None, start_at: None }
    }

    /// A comma-separated list of the parameters to expand
    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// a JQL query string
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// the maximum number of issues to return (defaults to 50)
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// whether to validate the JQL query
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// the list of fields to return for each issue
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// the index of the first issue to return (0-based)
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/search".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Performs a search using JQL.
#[derive(Clone)]
pub struct SearchIssuesPostRequest<'a> {
    client: &'a crate::core::Client,
    search_and_reconcile_request: SearchRequest,
}

impl<'a> SearchIssuesPostRequest<'a> {
    fn new(client: &'a crate::core::Client, search_and_reconcile_request: SearchRequest) -> Self {
        Self { client, search_and_reconcile_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/search".to_owned());

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.search_and_reconcile_request)?));

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages advance the offset. The stream ends at the last page,
    /// a known total, or an empty page.
    /// The stream also ends without an error at an empty page even if `total` promised more, since the
    /// listing can change while it is read.
    /// Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.search_and_reconcile_request.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.search_and_reconcile_request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Available since Jira Data Center 11.3.
#[derive(Clone)]
pub struct GetErrorRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetErrorRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/search/error/lookup".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Option<ResponseValue>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum SearchIssuesRequestExpandValue {
        RenderedFields => "renderedFields",
        Names => "names",
        Schema => "schema",
        Transitions => "transitions",
        Operations => "operations",
        Editmeta => "editmeta",
        Changelog => "changelog",
        VersionedRepresentations => "versionedRepresentations",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a comma-delimited string of values. The expand options are:
///
///  *  `renderedFields` Returns field values rendered in HTML format.
///  *  `names` Returns the display name of each field.
///  *  `schema` Returns the schema describing a field type.
///  *  `transitions` Returns all possible transitions for the issue.
///  *  `operations` Returns all possible operations for the issue.
///  *  `editmeta` Returns information about how each field can be edited.
///  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.
///  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version.
///
/// Examples: `"names,changelog"` Returns the display name of each field as well as a list of recent updates to an issue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SearchIssuesRequestExpand {
    One(SearchIssuesRequestExpandValue),
    Many(Vec<SearchIssuesRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The IssueSearch operations.
pub struct IssueSearchService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueSearchService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns lists of issues matching a query string. Use this resource to provide auto-completion suggestions when the user is looking for an issue using a word or string.
    ///
    /// This operation returns two lists:
    ///
    ///  *  `History Search` which includes issues from the user's history of created, edited, or viewed issues that contain the string in the `query` parameter.
    ///  *  `Current Search` which includes issues that match the JQL expression in `currentJQL` and contain the string in the `query` parameter.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_issue_picker_resource(&self) -> GetIssuePickerResourceRequest<'a> {
        GetIssuePickerResourceRequest::new(self.client)
    }

    /// Checks whether one or more issues would be returned by one or more JQL queries.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, issues are only matched against JQL queries where the user has:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn match_issues(&self, issues_and_jql_queries: IssuesAndJQLQueries) -> MatchIssuesRequest<'a> {
        MatchIssuesRequest::new(self.client, issues_and_jql_queries)
    }

    /// Provide an estimated count of the issues that match the [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned output. This endpoint requires JQL to be bounded.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn count_issues(&self, jql_count_request: JQLCountRequest) -> CountIssuesRequest<'a> {
        CountIssuesRequest::new(self.client, jql_count_request)
    }

    /// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need [read-after-write](https://developer.atlassian.com/cloud/jira/platform/search-and-reconcile/) consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.
    ///
    /// If the JQL query expression is too large to be encoded as a query parameter, use the [POST](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-search/#api-rest-api-3-search-post) version of this resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn search_issues(&self) -> SearchIssuesRequest<'a> {
        SearchIssuesRequest::new(self.client)
    }

    /// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need [read-after-write](https://developer.atlassian.com/cloud/jira/platform/search-and-reconcile/) consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    pub fn search_issues_post(
        &self,
        search_and_reconcile_request: SearchAndReconcileRequest,
    ) -> SearchIssuesPostRequest<'a> {
        SearchIssuesPostRequest::new(self.client, search_and_reconcile_request)
    }
}

/// Returns lists of issues matching a query string. Use this resource to provide auto-completion suggestions when the user is looking for an issue using a word or string.
///
/// This operation returns two lists:
///
///  *  `History Search` which includes issues from the user's history of created, edited, or viewed issues that contain the string in the `query` parameter.
///  *  `Current Search` which includes issues that match the JQL expression in `currentJQL` and contain the string in the `query` parameter.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetIssuePickerResourceRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    current_jql: Option<String>,
    current_issue_key: Option<String>,
    current_project_id: Option<String>,
    show_sub_tasks: Option<bool>,
    show_sub_task_parent: Option<bool>,
}

impl<'a> GetIssuePickerResourceRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            query: None,
            current_jql: None,
            current_issue_key: None,
            current_project_id: None,
            show_sub_tasks: None,
            show_sub_task_parent: None,
        }
    }

    /// A string to match against text fields in the issue such as title, description, or comments.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// A JQL query defining a list of issues to search for the query term. Note that `username` and `userkey` cannot be used as search terms for this parameter, due to privacy reasons. Use `accountId` instead.
    #[must_use]
    pub fn current_jql(mut self, value: impl Into<String>) -> Self {
        self.current_jql = Some(value.into());

        self
    }

    /// The key of an issue to exclude from search results. For example, the issue the user is viewing when they perform this query.
    #[must_use]
    pub fn current_issue_key(mut self, value: impl Into<String>) -> Self {
        self.current_issue_key = Some(value.into());

        self
    }

    /// The ID of a project that suggested issues must belong to.
    #[must_use]
    pub fn current_project_id(mut self, value: impl Into<String>) -> Self {
        self.current_project_id = Some(value.into());

        self
    }

    /// Indicate whether to include subtasks in the suggestions list.
    #[must_use]
    pub fn show_sub_tasks(mut self, value: bool) -> Self {
        self.show_sub_tasks = Some(value);

        self
    }

    /// When `currentIssueKey` is a subtask, whether to include the parent issue in the suggestions if it matches the query.
    #[must_use]
    pub fn show_sub_task_parent(mut self, value: bool) -> Self {
        self.show_sub_task_parent = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/issue/picker".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.current_jql {
            config.query.push(("currentJQL".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.current_issue_key {
            config.query.push(("currentIssueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.current_project_id {
            config.query.push(("currentProjectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.show_sub_tasks {
            config.query.push(("showSubTasks".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Checks whether one or more issues would be returned by one or more JQL queries.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None, however, issues are only matched against JQL queries where the user has:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project that the issue is in.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct MatchIssuesRequest<'a> {
    client: &'a crate::core::Client,
    issues_and_jql_queries: IssuesAndJQLQueries,
}

impl<'a> MatchIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, issues_and_jql_queries: IssuesAndJQLQueries) -> Self {
        Self { client, issues_and_jql_queries }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/jql/match".to_owned());

        let body = match serde_json::to_value(&self.issues_and_jql_queries)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueMatches> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Provide an estimated count of the issues that match the [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned output. This endpoint requires JQL to be bounded.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct CountIssuesRequest<'a> {
    client: &'a crate::core::Client,
    jql_count_request: JQLCountRequest,
}

impl<'a> CountIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, jql_count_request: JQLCountRequest) -> Self {
        Self { client, jql_count_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/3/search/approximate-count".to_owned(),
        );

        let body = match serde_json::to_value(&self.jql_count_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<JQLCountResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need [read-after-write](https://developer.atlassian.com/cloud/jira/platform/search-and-reconcile/) consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.
///
/// If the JQL query expression is too large to be encoded as a query parameter, use the [POST](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-search/#api-rest-api-3-search-post) version of this resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct SearchIssuesRequest<'a> {
    client: &'a crate::core::Client,
    jql: Option<String>,
    next_page_token: Option<String>,
    max_results: Option<i64>,
    fields: Option<Vec<String>>,
    expand: Option<SearchIssuesRequestExpand>,
    properties: Option<Vec<String>>,
    fields_by_keys: Option<bool>,
    fail_fast: Option<bool>,
    reconcile_issues: Option<Vec<i64>>,
    include_archived_projects: Option<bool>,
}

impl<'a> SearchIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            jql: None,
            next_page_token: None,
            max_results: None,
            fields: None,
            expand: None,
            properties: None,
            fields_by_keys: None,
            fail_fast: None,
            reconcile_issues: None,
            include_archived_projects: None,
        }
    }

    /// A [JQL](https://confluence.atlassian.com/x/egORLQ) expression. For performance reasons, this parameter requires a bounded query. A bounded query is a query with a search restriction.
    ///
    ///  *  Example of an unbounded query: `order by key desc`.
    ///  *  Example of a bounded query: `assignee = currentUser() order by key`.
    ///
    /// Additionally, `orderBy` clause can contain a maximum of 7 fields.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues.
    ///
    /// Note: The `nextPageToken` field is **not included** in the response for the last page, indicating there is no next page.
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of items to return per page. To manage page size, API may return fewer items per page where a large number of fields or properties are requested. The greatest number of items returned per page is achieved when requesting `id` or `key` only. It returns max 5000 issues.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// A list of fields to return for each issue, use it to retrieve a subset of fields. This parameter accepts a comma-separated list. Expand options include:
    ///
    ///  *  `*all` Returns all fields.
    ///  *  `*navigable` Returns navigable fields.
    ///  *  `id` Returns only issue IDs.
    ///  *  Any issue field, prefixed with a minus to exclude.
    ///
    /// The default is `id`.
    ///
    /// Examples:
    ///
    ///  *  `summary,comment` Returns only the summary and comments fields only.
    ///  *  `-description` Returns all navigable (default) fields except description.
    ///  *  `*all,-comment` Returns all fields except comments.
    ///
    /// Multiple `fields` parameters can be included in a request.
    ///
    /// Note: By default, this resource returns IDs only. This differs from [GET issue](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issues/#api-rest-api-3-issue-issueIdOrKey-get) where the default is all fields.
    #[must_use]
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about issues in the response. Note that, unlike the majority of instances where `expand` is specified, `expand` is defined as a comma-delimited string of values. The expand options are:
    ///
    ///  *  `renderedFields` Returns field values rendered in HTML format.
    ///  *  `names` Returns the display name of each field.
    ///  *  `schema` Returns the schema describing a field type.
    ///  *  `transitions` Returns all possible transitions for the issue.
    ///  *  `operations` Returns all possible operations for the issue.
    ///  *  `editmeta` Returns information about how each field can be edited.
    ///  *  `changelog` Returns a list of recent updates to an issue, sorted by date, starting from the most recent.
    ///  *  `versionedRepresentations` Instead of `fields`, returns `versionedRepresentations` a JSON array containing each version of a field's value, with the highest numbered item representing the most recent version.
    ///
    /// Examples: `"names,changelog"` Returns the display name of each field as well as a list of recent updates to an issue.
    #[must_use]
    pub fn expand(mut self, value: SearchIssuesRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// A list of up to 5 issue properties to include in the results. This parameter accepts a comma-separated list.
    #[must_use]
    pub fn properties(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.properties = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// Reference fields by their key (rather than ID). The default is `false`.
    #[must_use]
    pub fn fields_by_keys(mut self, value: bool) -> Self {
        self.fields_by_keys = Some(value);

        self
    }

    /// Fail this request early if we can't retrieve all field data.
    #[must_use]
    pub fn fail_fast(mut self, value: bool) -> Self {
        self.fail_fast = Some(value);

        self
    }

    /// Strong consistency issue ids to be reconciled with search results. Accepts max 50 ids. This list of ids should be consistent with each paginated request across different pages.
    #[must_use]
    pub fn reconcile_issues(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.reconcile_issues = Some(value.into_iter().collect());

        self
    }

    /// Whether to also return issues that belong to [archived projects](https://support.atlassian.com/jira-cloud-administration/docs/archive-a-project/). Issues in archived projects are excluded by default. Setting this to `true` returns them alongside issues from active projects; the *Browse projects* permission is still required on the archived project. The default is `false`.
    #[must_use]
    pub fn include_archived_projects(mut self, value: bool) -> Self {
        self.include_archived_projects = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/search/jql".to_owned());

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.properties {
            config.query.push(("properties".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.fields_by_keys {
            config.query.push(("fieldsByKeys".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fail_fast {
            config.query.push(("failFast".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.reconcile_issues {
            config.query.push(("reconcileIssues".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.include_archived_projects {
            config
                .query
                .push(("includeArchivedProjects".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchAndReconcileResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Searches for issues using [JQL](https://confluence.atlassian.com/x/egORLQ). Recent updates might not be immediately visible in the returned search results. If you need [read-after-write](https://developer.atlassian.com/cloud/jira/platform/search-and-reconcile/) consistency, you can utilize the `reconcileIssues` parameter to ensure stronger consistency assurances. This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Issues are included in the response where the user has:
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project containing the issue.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
#[derive(Clone)]
pub struct SearchIssuesPostRequest<'a> {
    client: &'a crate::core::Client,
    search_and_reconcile_request: SearchAndReconcileRequest,
}

impl<'a> SearchIssuesPostRequest<'a> {
    fn new(client: &'a crate::core::Client, search_and_reconcile_request: SearchAndReconcileRequest) -> Self {
        Self { client, search_and_reconcile_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/search/jql".to_owned());

        let body = match serde_json::to_value(&self.search_and_reconcile_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchAndReconcileResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

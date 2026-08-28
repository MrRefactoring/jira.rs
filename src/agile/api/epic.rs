// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// A comma-separated list of the parameters to expand.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetIssuesWithoutEpicRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// A comma-separated list of the parameters to expand.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetIssuesForEpicRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Epic operations.
pub struct EpicService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> EpicService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Removes issues from epics. The user needs to have the edit issue permission for all issue they want to remove from epics. The maximum number of issues that can be moved in one operation is 50. **Note:** This operation does not work for epics in next-gen projects. Instead, update the issue using `{ fields: { parent: {} } }`
    pub fn remove_issues_from_epic(
        &self,
        issues: impl IntoIterator<Item = impl Into<String>>,
    ) -> RemoveIssuesFromEpicRequest<'a> {
        RemoveIssuesFromEpicRequest::new(self.client, issues)
    }

    /// Returns all issues that do not belong to any epic. Result pagination is token based, using `nextPageToken` and `maxResults`. This only includes issues that the user has permission to view. Issues returned from this resource include Software project fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank. **Note:** If you are querying a Team Managed project, do not use this operation. Instead, search for issues that don't belong to an epic by using the [Search for issues using JQL enhanced search](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-search/#api-rest-api-3-search-jql-get) operation in the Jira platform REST API. Build your JQL query using the `parent is empty` clause. For more information on the `parent` JQL field, see [Advanced searching](https://confluence.atlassian.com/x/dAiiLQ#Advancedsearching-fieldsreference-Parent).
    pub fn get_issues_without_epic(&self) -> GetIssuesWithoutEpicRequest<'a> {
        GetIssuesWithoutEpicRequest::new(self.client)
    }

    /// Returns the epic for a given epic ID. This epic will only be returned if the user has permission to view it. **Note:** This operation does not work for epics in next-gen projects.
    pub fn get_epic(&self, epic_id_or_key: impl Into<String>) -> GetEpicRequest<'a> {
        GetEpicRequest::new(self.client, epic_id_or_key)
    }

    /// Performs a partial update of the epic. A partial update means that fields not present in the request JSON will not be updated. Valid values for color are `color_1` to `color_9`. **Note:** This operation does not work for epics in next-gen projects.
    pub fn partially_update_epic(
        &self,
        epic_id_or_key: impl Into<String>,
        epic_update: EpicUpdate,
    ) -> PartiallyUpdateEpicRequest<'a> {
        PartiallyUpdateEpicRequest::new(self.client, epic_id_or_key, epic_update)
    }

    /// Moves issues to an epic, for a given epic id. Issues can be only in a single epic at the same time. That means that already assigned issues to an epic, will not be assigned to the previous epic anymore. The user needs to have the edit issue permission for all issue they want to move and to the epic. The maximum number of issues that can be moved in one operation is 50. **Note:** This operation does not work for epics in next-gen projects.
    pub fn move_issues_to_epic(&self, epic_id_or_key: impl Into<String>) -> MoveIssuesToEpicRequest<'a> {
        MoveIssuesToEpicRequest::new(self.client, epic_id_or_key)
    }

    /// Returns all issues that belong to the epic, for the given epic ID. Result pagination is token based, using `nextPageToken` and `maxResults`. This only includes issues that the user has permission to view. Issues returned from this resource include Software project fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank. **Note:** If you are querying a Team Managed project, do not use this operation. Instead, search for issues that belong to an epic by using the [Search for issues using JQL enhanced search](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-search/#api-rest-api-3-search-jql-get) operation in the Jira platform REST API. Build your JQL query using the `parent` clause. For more information on the `parent` JQL field, see [Advanced searching](https://confluence.atlassian.com/x/dAiiLQ#Advancedsearching-fieldsreference-Parent).
    pub fn get_issues_for_epic(&self, epic_id_or_key: impl Into<String>) -> GetIssuesForEpicRequest<'a> {
        GetIssuesForEpicRequest::new(self.client, epic_id_or_key)
    }

    /// Moves (ranks) an epic before or after a given epic.
    ///
    /// If rankCustomFieldId is not defined, the default rank field will be used.
    ///
    /// **Note:** This operation does not work for epics in next-gen projects.
    pub fn rank_epics(
        &self,
        epic_id_or_key: impl Into<String>,
        epic_rank_request: EpicRankRequest,
    ) -> RankEpicsRequest<'a> {
        RankEpicsRequest::new(self.client, epic_id_or_key, epic_rank_request)
    }
}

/// Removes issues from epics. The user needs to have the edit issue permission for all issue they want to remove from epics. The maximum number of issues that can be moved in one operation is 50. **Note:** This operation does not work for epics in next-gen projects. Instead, update the issue using `{ fields: { parent: {} } }`
pub struct RemoveIssuesFromEpicRequest<'a> {
    client: &'a crate::core::Client,
    issues: Vec<String>,
}

impl<'a> RemoveIssuesFromEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, issues: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { client, issues: issues.into_iter().map(Into::into).collect() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/agile/1.0/epic/none/issue".to_owned());

        let mut body = serde_json::Map::new();

        body.insert("issues".to_owned(), serde_json::to_value(&self.issues)?);

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

/// Returns all issues that do not belong to any epic. Result pagination is token based, using `nextPageToken` and `maxResults`. This only includes issues that the user has permission to view. Issues returned from this resource include Software project fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank. **Note:** If you are querying a Team Managed project, do not use this operation. Instead, search for issues that don't belong to an epic by using the [Search for issues using JQL enhanced search](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-search/#api-rest-api-3-search-jql-get) operation in the Jira platform REST API. Build your JQL query using the `parent is empty` clause. For more information on the `parent` JQL field, see [Advanced searching](https://confluence.atlassian.com/x/dAiiLQ#Advancedsearching-fieldsreference-Parent).
pub struct GetIssuesWithoutEpicRequest<'a> {
    client: &'a crate::core::Client,
    next_page_token: Option<String>,
    max_results: Option<i64>,
    reconcile_issues: Option<Vec<i64>>,
    jql: Option<String>,
    validate_query: Option<bool>,
    fields: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    expand: Option<GetIssuesWithoutEpicRequestExpand>,
}

impl<'a> GetIssuesWithoutEpicRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            next_page_token: None,
            max_results: None,
            reconcile_issues: None,
            jql: None,
            validate_query: None,
            fields: None,
            expand: None,
        }
    }

    /// The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues.
    ///
    /// Note: The `nextPageToken` field is **not included** in the response for the last page, indicating there is no next page.
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of items to return per page. To manage page size, the API may return fewer items per page where there is a large number of fields or properties returned. It returns max 5000 issues.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Strong consistency issue IDs to be reconciled with search results. Accepts max 50 IDs. This list of IDs should be consistent with each paginated request across different pages.
    #[must_use]
    pub fn reconcile_issues(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.reconcile_issues = Some(value.into_iter().collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    /// Note that `username` and `userkey` can't be used as search terms for this parameter due to privacy reasons. Use `accountId` instead.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Software project fields are returned.
    #[must_use]
    pub fn fields(
        mut self,
        value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.fields = Some(value.into_iter().collect());

        self
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: GetIssuesWithoutEpicRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/software/1.0/epic/none/issue".to_owned());

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.reconcile_issues {
            config.query.push(("reconcileIssues".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SoftwareIssueResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the epic for a given epic ID. This epic will only be returned if the user has permission to view it. **Note:** This operation does not work for epics in next-gen projects.
pub struct GetEpicRequest<'a> {
    client: &'a crate::core::Client,
    epic_id_or_key: String,
}

impl<'a> GetEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, epic_id_or_key: impl Into<String>) -> Self {
        Self { client, epic_id_or_key: epic_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/epic/{}", crate::core::encode_path_segment(&self.epic_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Epic> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Performs a partial update of the epic. A partial update means that fields not present in the request JSON will not be updated. Valid values for color are `color_1` to `color_9`. **Note:** This operation does not work for epics in next-gen projects.
pub struct PartiallyUpdateEpicRequest<'a> {
    client: &'a crate::core::Client,
    epic_id_or_key: String,
    epic_update: EpicUpdate,
}

impl<'a> PartiallyUpdateEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, epic_id_or_key: impl Into<String>, epic_update: EpicUpdate) -> Self {
        Self { client, epic_id_or_key: epic_id_or_key.into(), epic_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/epic/{}", crate::core::encode_path_segment(&self.epic_id_or_key)),
        );

        let body = match serde_json::to_value(&self.epic_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Epic> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Moves issues to an epic, for a given epic id. Issues can be only in a single epic at the same time. That means that already assigned issues to an epic, will not be assigned to the previous epic anymore. The user needs to have the edit issue permission for all issue they want to move and to the epic. The maximum number of issues that can be moved in one operation is 50. **Note:** This operation does not work for epics in next-gen projects.
pub struct MoveIssuesToEpicRequest<'a> {
    client: &'a crate::core::Client,
    epic_id_or_key: String,
    issues: Option<Vec<String>>,
}

impl<'a> MoveIssuesToEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, epic_id_or_key: impl Into<String>) -> Self {
        Self { client, epic_id_or_key: epic_id_or_key.into(), issues: None }
    }

    #[must_use]
    pub fn issues(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.issues = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/epic/{}/issue", crate::core::encode_path_segment(&self.epic_id_or_key)),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.issues {
            body.insert("issues".to_owned(), serde_json::to_value(value)?);
        }

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

/// Returns all issues that belong to the epic, for the given epic ID. Result pagination is token based, using `nextPageToken` and `maxResults`. This only includes issues that the user has permission to view. Issues returned from this resource include Software project fields, like sprint, closedSprints, flagged, and epic. By default, the returned issues are ordered by rank. **Note:** If you are querying a Team Managed project, do not use this operation. Instead, search for issues that belong to an epic by using the [Search for issues using JQL enhanced search](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-search/#api-rest-api-3-search-jql-get) operation in the Jira platform REST API. Build your JQL query using the `parent` clause. For more information on the `parent` JQL field, see [Advanced searching](https://confluence.atlassian.com/x/dAiiLQ#Advancedsearching-fieldsreference-Parent).
pub struct GetIssuesForEpicRequest<'a> {
    client: &'a crate::core::Client,
    epic_id_or_key: String,
    next_page_token: Option<String>,
    max_results: Option<i64>,
    reconcile_issues: Option<Vec<i64>>,
    jql: Option<String>,
    validate_query: Option<bool>,
    fields: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    expand: Option<GetIssuesForEpicRequestExpand>,
}

impl<'a> GetIssuesForEpicRequest<'a> {
    fn new(client: &'a crate::core::Client, epic_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            epic_id_or_key: epic_id_or_key.into(),
            next_page_token: None,
            max_results: None,
            reconcile_issues: None,
            jql: None,
            validate_query: None,
            fields: None,
            expand: None,
        }
    }

    /// The token for a page to fetch that is not the first page. The first page has a `nextPageToken` of `null`. Use the `nextPageToken` to fetch the next page of issues.
    ///
    /// Note: The `nextPageToken` field is **not included** in the response for the last page, indicating there is no next page.
    #[must_use]
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());

        self
    }

    /// The maximum number of items to return per page. To manage page size, the API may return fewer items per page where there is a large number of fields or properties returned. It returns max 5000 issues.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Strong consistency issue IDs to be reconciled with search results. Accepts max 50 IDs. This list of IDs should be consistent with each paginated request across different pages.
    #[must_use]
    pub fn reconcile_issues(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.reconcile_issues = Some(value.into_iter().collect());

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    /// Note that `username` and `userkey` can't be used as search terms for this parameter due to privacy reasons. Use `accountId` instead.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Software project fields are returned.
    #[must_use]
    pub fn fields(
        mut self,
        value: impl IntoIterator<Item = std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        self.fields = Some(value.into_iter().collect());

        self
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: GetIssuesForEpicRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/software/1.0/epic/{}/issue", crate::core::encode_path_segment(&self.epic_id_or_key)),
        );

        if let Some(value) = &self.next_page_token {
            config.query.push(("nextPageToken".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.reconcile_issues {
            config.query.push(("reconcileIssues".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SoftwareIssueResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Moves (ranks) an epic before or after a given epic.
///
/// If rankCustomFieldId is not defined, the default rank field will be used.
///
/// **Note:** This operation does not work for epics in next-gen projects.
pub struct RankEpicsRequest<'a> {
    client: &'a crate::core::Client,
    epic_id_or_key: String,
    epic_rank_request: EpicRankRequest,
}

impl<'a> RankEpicsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        epic_id_or_key: impl Into<String>,
        epic_rank_request: EpicRankRequest,
    ) -> Self {
        Self { client, epic_id_or_key: epic_id_or_key.into(), epic_rank_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/agile/1.0/epic/{}/rank", crate::core::encode_path_segment(&self.epic_id_or_key)),
        );

        let body = match serde_json::to_value(&self.epic_rank_request)? {
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

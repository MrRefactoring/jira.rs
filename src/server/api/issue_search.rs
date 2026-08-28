// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// A comma-separated list of the parameters to expand
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SearchRequest2Expand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// the list of fields to return for each issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SearchRequest2Fields {
    One(String),
    Many(Vec<String>),
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
    pub fn search(&self) -> SearchRequest2<'a> {
        SearchRequest2::new(self.client)
    }

    /// Performs a search using JQL.
    pub fn search_using_search_request(&self, search_request: SearchRequest) -> SearchUsingSearchRequestRequest<'a> {
        SearchUsingSearchRequestRequest::new(self.client, search_request)
    }

    /// Available since Jira Data Center 11.3.
    pub fn get_error(&self) -> GetErrorRequest<'a> {
        GetErrorRequest::new(self.client)
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
pub struct SearchRequest2<'a> {
    client: &'a crate::core::Client,
    expand: Option<SearchRequest2Expand>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    fields: Option<SearchRequest2Fields>,
    start_at: Option<i64>,
}

impl<'a> SearchRequest2<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, jql: None, max_results: None, validate_query: None, fields: None, start_at: None }
    }

    /// A comma-separated list of the parameters to expand
    #[must_use]
    pub fn expand(mut self, value: SearchRequest2Expand) -> Self {
        self.expand = Some(value);

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
    pub fn fields(mut self, value: SearchRequest2Fields) -> Self {
        self.fields = Some(value);

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
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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
            config.query.push(("fields".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
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
pub struct SearchUsingSearchRequestRequest<'a> {
    client: &'a crate::core::Client,
    search_request: SearchRequest,
}

impl<'a> SearchUsingSearchRequestRequest<'a> {
    fn new(client: &'a crate::core::Client, search_request: SearchRequest) -> Self {
        Self { client, search_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/search".to_owned());

        let body = match serde_json::to_value(&self.search_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
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
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

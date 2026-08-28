// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Knowledgebase operations.
pub struct KnowledgebaseService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> KnowledgebaseService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns articles which match the given query string across all service desks.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the [customer portal](https://confluence.atlassian.com/servicedeskcloud/configuring-the-customer-portal-732528918.html).
    pub fn get_articles(&self, query: impl Into<String>, highlight: bool) -> GetArticlesRequest<'a> {
        GetArticlesRequest::new(self.client, query, highlight)
    }

    pub fn view_article(&self, page_id: i64) -> ViewArticleRequest<'a> {
        ViewArticleRequest::new(self.client, page_id)
    }
}

/// Returns articles which match the given query string across all service desks.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/service-desk/rest/intro#permissions) required**: Permission to access the [customer portal](https://confluence.atlassian.com/servicedeskcloud/configuring-the-customer-portal-732528918.html).
#[derive(Clone)]
pub struct GetArticlesRequest<'a> {
    client: &'a crate::core::Client,
    query: String,
    highlight: bool,
    start: Option<i64>,
    limit: Option<i64>,
    cursor: Option<String>,
    prev: Option<bool>,
}

impl<'a> GetArticlesRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>, highlight: bool) -> Self {
        Self { client, query: query.into(), highlight, start: None, limit: None, cursor: None, prev: None }
    }

    /// (Deprecated) The starting index of the returned objects. Base index: 0.
    #[deprecated(note = "(Deprecated) The starting index of the returned objects.")]
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// Pointer to a set of search results, returned as part of the next or prev URL from the previous search call.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// Should navigate to the previous page. Defaulted to false. Set to true as part of prev URL from the previous search call.
    #[must_use]
    pub fn prev(mut self, value: bool) -> Self {
        self.prev = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/servicedeskapi/knowledgebase/article".to_owned(),
        );

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

        config.query.push(("highlight".to_owned(), crate::core::QueryValue::Scalar(self.highlight.to_string())));

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.prev {
            config.query.push(("prev".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Article>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

#[derive(Clone)]
pub struct ViewArticleRequest<'a> {
    client: &'a crate::core::Client,
    page_id: i64,
}

impl<'a> ViewArticleRequest<'a> {
    fn new(client: &'a crate::core::Client, page_id: i64) -> Self {
        Self { client, page_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/knowledgebase/article/view/{}", self.page_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

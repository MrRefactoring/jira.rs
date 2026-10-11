// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssuePriorities operations.
pub struct IssuePrioritiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssuePrioritiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all issue priorities
    pub fn get_priorities(&self) -> GetPrioritiesRequest<'a> {
        GetPrioritiesRequest::new(self.client)
    }

    /// Returns a page with list of issue priorities whose names (or their translations) match query
    pub fn get_priorities_paginated(&self) -> GetPrioritiesPaginatedRequest<'a> {
        GetPrioritiesPaginatedRequest::new(self.client)
    }

    /// Returns an issue priority
    pub fn get_priority(&self, id: impl Into<String>) -> GetPriorityRequest<'a> {
        GetPriorityRequest::new(self.client, id)
    }
}

/// Returns a list of all issue priorities
#[derive(Clone)]
pub struct GetPrioritiesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetPrioritiesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/priority".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Priority>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a page with list of issue priorities whose names (or their translations) match query
#[derive(Clone)]
pub struct GetPrioritiesPaginatedRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: Option<String>,
    project_ids: Option<Vec<i64>>,
    start_at: Option<i64>,
}

impl<'a> GetPrioritiesPaginatedRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, query: None, project_ids: None, start_at: None }
    }

    /// how many results on the page should be included. Defaults to 100
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// query that should match priority name or its translation
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// the list of project ids to filter priorities
    #[must_use]
    pub fn project_ids(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.project_ids = Some(value.into_iter().collect());

        self
    }

    /// the page offset, if not specified then defaults to 0
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/priority/page".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_ids {
            config.query.push(("projectIds".to_owned(), crate::core::QueryValue::from_serializable(value)?));
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Priority>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.values;
            Ok(crate::core::PageStep::offset(items, page.start_at, page.is_last, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Priority>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns an issue priority
#[derive(Clone)]
pub struct GetPriorityRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetPriorityRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/priority/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Priority> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowStatuses operations.
pub struct WorkflowStatusesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowStatusesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all statuses
    pub fn get_statuses(&self) -> GetStatusesRequest<'a> {
        GetStatusesRequest::new(self.client)
    }

    /// Returns paginated list of filtered statuses
    pub fn get_paginated_statuses(&self) -> GetPaginatedStatusesRequest<'a> {
        GetPaginatedStatusesRequest::new(self.client)
    }

    /// Returns a full representation of the Status having the given id or name.
    pub fn get_status(&self, id_or_name: impl Into<String>) -> GetStatusRequest<'a> {
        GetStatusRequest::new(self.client, id_or_name)
    }
}

/// Returns a list of all statuses
#[derive(Clone)]
pub struct GetStatusesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/status".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<StatusJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns paginated list of filtered statuses
#[derive(Clone)]
pub struct GetPaginatedStatusesRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_ids: Option<Vec<String>>,
    max_results: Option<i64>,
    query: Option<String>,
    project_ids: Option<Vec<i64>>,
    start_at: Option<i64>,
}

impl<'a> GetPaginatedStatusesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, issue_type_ids: None, max_results: None, query: None, project_ids: None, start_at: None }
    }

    /// The list of issue type ids to filter statuses.
    #[must_use]
    pub fn issue_type_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.issue_type_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The maximum number of statuses to return.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The string that status names will be matched with.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The list of project ids to filter statuses.
    #[must_use]
    pub fn project_ids(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.project_ids = Some(value.into_iter().collect());

        self
    }

    /// The index of the first status to return.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/status/page".to_owned());

        if let Some(value) = &self.issue_type_ids {
            config.query.push(("issueTypeIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

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
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<StatusJson>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start_at = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<StatusJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of the Status having the given id or name.
#[derive(Clone)]
pub struct GetStatusRequest<'a> {
    client: &'a crate::core::Client,
    id_or_name: String,
}

impl<'a> GetStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, id_or_name: impl Into<String>) -> Self {
        Self { client, id_or_name: id_or_name.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/status/{}", crate::core::encode_path_segment(&self.id_or_name)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

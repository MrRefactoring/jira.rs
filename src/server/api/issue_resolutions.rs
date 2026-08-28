// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueResolutions operations.
pub struct IssueResolutionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueResolutionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all resolutions.
    pub fn get_resolutions(&self) -> GetResolutionsRequest<'a> {
        GetResolutionsRequest::new(self.client)
    }

    /// Returns paginated list of filtered resolutions.
    pub fn get_paginated_resolutions(&self) -> GetPaginatedResolutionsRequest<'a> {
        GetPaginatedResolutionsRequest::new(self.client)
    }

    /// Returns a resolution.
    pub fn get_resolution(&self, id: impl Into<String>) -> GetResolutionRequest<'a> {
        GetResolutionRequest::new(self.client, id)
    }
}

/// Returns a list of all resolutions.
pub struct GetResolutionsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetResolutionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/resolution".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ResolutionJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns paginated list of filtered resolutions.
pub struct GetPaginatedResolutionsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: Option<String>,
    start_at: Option<i64>,
}

impl<'a> GetPaginatedResolutionsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, max_results: None, query: None, start_at: None }
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

    /// The index of the first status to return.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/resolution/page".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Resolution> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a resolution.
pub struct GetResolutionRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetResolutionRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/resolution/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ResolutionJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

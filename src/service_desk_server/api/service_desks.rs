// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ServiceDesks operations.
pub struct ServiceDesksService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ServiceDesksService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the service project for a given service project Id.
    pub fn get_service_desk_by_id(&self, service_desk_id: impl Into<String>) -> GetServiceDeskByIdRequest<'a> {
        GetServiceDeskByIdRequest::new(self.client, service_desk_id)
    }

    /// Returns all service projects in the Jira Service Management application with the option to include archived service projects.
    pub fn get_service_desks(&self) -> GetServiceDesksRequest<'a> {
        GetServiceDesksRequest::new(self.client)
    }
}

/// Returns the service project for a given service project Id.
#[derive(Clone)]
pub struct GetServiceDeskByIdRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
}

impl<'a> GetServiceDeskByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/servicedesk/{}", crate::core::encode_path_segment(&self.service_desk_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ServiceDesk> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all service projects in the Jira Service Management application with the option to include archived service projects.
#[derive(Clone)]
pub struct GetServiceDesksRequest<'a> {
    client: &'a crate::core::Client,
    include_archived: Option<String>,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetServiceDesksRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, include_archived: None, start: None, limit: None }
    }

    /// The option to include archived service project. False by default.
    #[must_use]
    pub fn include_archived(mut self, value: impl Into<String>) -> Self {
        self.include_archived = Some(value.into());

        self
    }

    /// The starting index of the returned objects. Base index: 0.
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

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/servicedesk".to_owned());

        if let Some(value) = &self.include_archived {
            config.query.push(("includeArchived".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<ServiceDesk>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ServiceDesk>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowStatusCategories operations.
pub struct WorkflowStatusCategoriesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowStatusCategoriesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all status categories
    pub fn get_status_categories(&self) -> GetStatusCategoriesRequest<'a> {
        GetStatusCategoriesRequest::new(self.client)
    }

    /// Returns a full representation of the StatusCategory having the given id or key
    pub fn get_status_category(&self, id_or_key: impl Into<String>) -> GetStatusCategoryRequest<'a> {
        GetStatusCategoryRequest::new(self.client, id_or_key)
    }
}

/// Returns a list of all status categories
pub struct GetStatusCategoriesRequest<'a> {
    client: &'a crate::core::Client,
    request: Option<String>,
    uri_info: Option<String>,
}

impl<'a> GetStatusCategoriesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, request: None, uri_info: None }
    }

    /// a Request
    #[must_use]
    pub fn request(mut self, value: impl Into<String>) -> Self {
        self.request = Some(value.into());

        self
    }

    /// a UriInfo
    #[must_use]
    pub fn uri_info(mut self, value: impl Into<String>) -> Self {
        self.uri_info = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/statuscategory".to_owned());

        if let Some(value) = &self.request {
            config.query.push(("request".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.uri_info {
            config.query.push(("uriInfo".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<StatusCategoryJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a full representation of the StatusCategory having the given id or key
pub struct GetStatusCategoryRequest<'a> {
    client: &'a crate::core::Client,
    id_or_key: String,
}

impl<'a> GetStatusCategoryRequest<'a> {
    fn new(client: &'a crate::core::Client, id_or_key: impl Into<String>) -> Self {
        Self { client, id_or_key: id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/statuscategory/{}", self.id_or_key),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusCategoryJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Portals operations.
pub struct PortalsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PortalsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get a portal with the specified ID.
    pub fn get_portal(&self, portal_id: impl Into<String>) -> GetPortalRequest<'a> {
        GetPortalRequest::new(self.client, portal_id)
    }

    /// Get a portal for the project by key.
    pub fn get_portal_by_project_key(&self, project_key: impl Into<String>) -> GetPortalByProjectKeyRequest<'a> {
        GetPortalByProjectKeyRequest::new(self.client, project_key)
    }

    /// Get all portals. Returns a maximum of 50 portals per page.
    pub fn get_portals(&self) -> GetPortalsRequest<'a> {
        GetPortalsRequest::new(self.client)
    }
}

/// Get a portal with the specified ID.
pub struct GetPortalRequest<'a> {
    client: &'a crate::core::Client,
    portal_id: String,
}

impl<'a> GetPortalRequest<'a> {
    fn new(client: &'a crate::core::Client, portal_id: impl Into<String>) -> Self {
        Self { client, portal_id: portal_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/portals/{}", self.portal_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Portal> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get a portal for the project by key.
pub struct GetPortalByProjectKeyRequest<'a> {
    client: &'a crate::core::Client,
    project_key: String,
}

impl<'a> GetPortalByProjectKeyRequest<'a> {
    fn new(client: &'a crate::core::Client, project_key: impl Into<String>) -> Self {
        Self { client, project_key: project_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/portals/project/{}", self.project_key),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Portal> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get all portals. Returns a maximum of 50 portals per page.
pub struct GetPortalsRequest<'a> {
    client: &'a crate::core::Client,
    start: Option<f64>,
    limit: Option<f64>,
}

impl<'a> GetPortalsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/portals".to_owned());

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Portal>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

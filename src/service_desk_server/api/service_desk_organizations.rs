// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ServiceDeskOrganizations operations.
pub struct ServiceDeskOrganizationsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ServiceDeskOrganizationsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all organizations within a service project for a given service project id.
    pub fn get_service_desk_organizations(
        &self,
        service_desk_id: impl Into<String>,
    ) -> GetServiceDeskOrganizationsRequest<'a> {
        GetServiceDeskOrganizationsRequest::new(self.client, service_desk_id)
    }

    /// Adds an organization to a service project for a given service project id and organization id.
    pub fn add_organization(&self, service_desk_id: impl Into<String>) -> AddOrganizationRequest<'a> {
        AddOrganizationRequest::new(self.client, service_desk_id)
    }

    /// Removes an organization from a service project for a given service project id and organization id.
    pub fn remove_organization(&self, service_desk_id: impl Into<String>) -> RemoveOrganizationRequest<'a> {
        RemoveOrganizationRequest::new(self.client, service_desk_id)
    }
}

/// Returns all organizations within a service project for a given service project id.
pub struct GetServiceDeskOrganizationsRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    start: Option<f64>,
    limit: Option<f64>,
}

impl<'a> GetServiceDeskOrganizationsRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), start: None, limit: None }
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
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/servicedeskapi/servicedesk/{}/organization", self.service_desk_id),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Organization>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds an organization to a service project for a given service project id and organization id.
pub struct AddOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    organization_service_desk_update: Option<OrganizationServiceDeskUpdate>,
}

impl<'a> AddOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), organization_service_desk_update: None }
    }

    #[must_use]
    pub fn organization_service_desk_update(mut self, value: OrganizationServiceDeskUpdate) -> Self {
        self.organization_service_desk_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/servicedeskapi/servicedesk/{}/organization", self.service_desk_id),
        );

        let body = match serde_json::to_value(&self.organization_service_desk_update)? {
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

/// Removes an organization from a service project for a given service project id and organization id.
pub struct RemoveOrganizationRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    organization_service_desk_update: Option<OrganizationServiceDeskUpdate>,
}

impl<'a> RemoveOrganizationRequest<'a> {
    fn new(client: &'a crate::core::Client, service_desk_id: impl Into<String>) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), organization_service_desk_update: None }
    }

    #[must_use]
    pub fn organization_service_desk_update(mut self, value: OrganizationServiceDeskUpdate) -> Self {
        self.organization_service_desk_update = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/servicedeskapi/servicedesk/{}/organization", self.service_desk_id),
        );

        let body = match serde_json::to_value(&self.organization_service_desk_update)? {
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

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Dashboards operations.
pub struct DashboardsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DashboardsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all dashboards, optionally filtering them.
    pub fn get_all_dashboards(&self) -> GetAllDashboardsRequest<'a> {
        GetAllDashboardsRequest::new(self.client)
    }

    /// Returns the keys of all properties for the dashboard item identified by the id.
    pub fn get_dashboard_item_property_keys(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> GetDashboardItemPropertyKeysRequest<'a> {
        GetDashboardItemPropertyKeysRequest::new(self.client, dashboard_id, item_id)
    }

    /// Returns the value of the property with a given key from the dashboard item identified by the id.
    pub fn get_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetDashboardItemPropertyRequest<'a> {
        GetDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key)
    }

    /// Sets the value of the property with a given key on the dashboard item identified by the id.
    pub fn set_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetDashboardItemPropertyRequest<'a> {
        SetDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key, body)
    }

    /// Removes the property from the dashboard item identified by the key or by the id.
    pub fn delete_dashboard_item_property(
        &self,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteDashboardItemPropertyRequest<'a> {
        DeleteDashboardItemPropertyRequest::new(self.client, dashboard_id, item_id, property_key)
    }

    /// Returns a single dashboard.
    pub fn get_dashboard(&self, id: impl Into<String>) -> GetDashboardRequest<'a> {
        GetDashboardRequest::new(self.client, id)
    }
}

/// Returns a list of all dashboards, optionally filtering them.
#[derive(Clone)]
pub struct GetAllDashboardsRequest<'a> {
    client: &'a crate::core::Client,
    filter: Option<String>,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetAllDashboardsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, filter: None, max_results: None, start_at: None }
    }

    /// An optional filter that is applied to the list of dashboards.
    #[must_use]
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());

        self
    }

    /// A hint as to the maximum number of dashboards to return in each call.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The index of the first dashboard to return (0-based).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/dashboard".to_owned());

        if let Some(value) = &self.filter {
            config.query.push(("filter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Dashboard>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.dashboards.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PageOfDashboards> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the keys of all properties for the dashboard item identified by the id.
#[derive(Clone)]
pub struct GetDashboardItemPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
}

impl<'a> GetDashboardItemPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, dashboard_id: impl Into<String>, item_id: impl Into<String>) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/dashboard/{}/items/{}/properties",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the dashboard item identified by the id.
#[derive(Clone)]
pub struct GetDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
}

impl<'a> GetDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the property with a given key on the dashboard item identified by the id.
#[derive(Clone)]
pub struct SetDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            client,
            dashboard_id: dashboard_id.into(),
            item_id: item_id.into(),
            property_key: property_key.into(),
            body,
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Removes the property from the dashboard item identified by the key or by the id.
#[derive(Clone)]
pub struct DeleteDashboardItemPropertyRequest<'a> {
    client: &'a crate::core::Client,
    dashboard_id: String,
    item_id: String,
    property_key: String,
}

impl<'a> DeleteDashboardItemPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        dashboard_id: impl Into<String>,
        item_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> Self {
        Self { client, dashboard_id: dashboard_id.into(), item_id: item_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/dashboard/{}/items/{}/properties/{}",
                crate::core::encode_path_segment(&self.dashboard_id),
                crate::core::encode_path_segment(&self.item_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

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

/// Returns a single dashboard.
#[derive(Clone)]
pub struct GetDashboardRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetDashboardRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/dashboard/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Dashboard> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

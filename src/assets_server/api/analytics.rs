// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Analytics operations.
pub struct AnalyticsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AnalyticsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get the shape of the data held in each schema.
    pub fn get_schema_analytics(&self) -> GetSchemaAnalyticsRequest<'a> {
        GetSchemaAnalyticsRequest::new(self.client)
    }
}

/// Get the shape of the data held in each schema.
pub struct GetSchemaAnalyticsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetSchemaAnalyticsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/analytics/schema".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<SchemaStats>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Imports operations.
pub struct ImportsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ImportsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Start configured imports. To see an ongoing import see the Progress resource
    pub fn start_import(&self, id: impl Into<String>) -> StartImportRequest<'a> {
        StartImportRequest::new(self.client, id)
    }
}

/// Start configured imports. To see an ongoing import see the Progress resource
pub struct StartImportRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> StartImportRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/import/start/{}", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Progress> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Progress operations.
pub struct ProgressService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProgressService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Show ongoing import process
    pub fn get_import_progress(&self, id: impl Into<String>) -> GetImportProgressRequest<'a> {
        GetImportProgressRequest::new(self.client, id)
    }
}

/// Show ongoing import process
#[derive(Clone)]
pub struct GetImportProgressRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetImportProgressRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/progress/category/imports/{}", crate::core::encode_path_segment(&self.id)),
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

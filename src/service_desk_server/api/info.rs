// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Info operations.
pub struct InfoService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> InfoService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns runtime information about Jira Service Management. You do not need to be logged in to use this method.
    pub fn get_info(&self) -> GetInfoRequest<'a> {
        GetInfoRequest::new(self.client)
    }
}

/// Returns runtime information about Jira Service Management. You do not need to be logged in to use this method.
#[derive(Clone)]
pub struct GetInfoRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetInfoRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/servicedeskapi/info".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SoftwareInfo> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

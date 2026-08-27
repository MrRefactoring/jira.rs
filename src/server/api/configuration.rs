// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Configuration operations.
pub struct ConfigurationService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ConfigurationService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the information if the optional features in Jira are enabled or disabled. If the time tracking is enabled, it also returns the detailed information about time tracking configuration.
    pub fn get_configuration(&self) -> GetConfigurationRequest<'a> {
        GetConfigurationRequest::new(self.client)
    }
}

/// Returns the information if the optional features in Jira are enabled or disabled. If the time tracking is enabled, it also returns the detailed information about time tracking configuration.
pub struct GetConfigurationRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetConfigurationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/configuration".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Configuration> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

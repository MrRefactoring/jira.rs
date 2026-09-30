// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ReadOnlyMode operations.
pub struct ReadOnlyModeService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ReadOnlyModeService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns whether Jira is currently in read-only mode.
    ///
    /// Available since Jira Data Center 11.3, and in 10.3 LTS.
    pub fn get_read_only_mode(&self) -> GetReadOnlyModeRequest<'a> {
        GetReadOnlyModeRequest::new(self.client)
    }

    /// Enables or disables Jira read-only mode.
    ///
    /// Available since Jira Data Center 11.3, and in 10.3 LTS.
    pub fn update_read_only_mode(&self) -> UpdateReadOnlyModeRequest<'a> {
        UpdateReadOnlyModeRequest::new(self.client)
    }
}

/// Returns whether Jira is currently in read-only mode.
///
/// Available since Jira Data Center 11.3, and in 10.3 LTS.
#[derive(Clone)]
pub struct GetReadOnlyModeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetReadOnlyModeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/readonly-mode".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ReadOnlyModeStatus> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Enables or disables Jira read-only mode.
///
/// Available since Jira Data Center 11.3, and in 10.3 LTS.
#[derive(Clone)]
pub struct UpdateReadOnlyModeRequest<'a> {
    client: &'a crate::core::Client,
    read_only_mode_update_request: Option<ReadOnlyModeUpdateRequest>,
}

impl<'a> UpdateReadOnlyModeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, read_only_mode_update_request: None }
    }

    #[must_use]
    pub fn read_only_mode_update_request(mut self, value: ReadOnlyModeUpdateRequest) -> Self {
        self.read_only_mode_update_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/readonly-mode".to_owned());

        let body = match serde_json::to_value(&self.read_only_mode_update_request)? {
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

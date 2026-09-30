// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IndexConfiguration operations.
pub struct IndexConfigurationService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IndexConfigurationService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Check the integrity of the index.
    pub fn check_index_integrity(&self) -> CheckIndexIntegrityRequest<'a> {
        CheckIndexIntegrityRequest::new(self.client)
    }

    /// Get the path to the current Assets Index.
    pub fn get_index_path(&self) -> GetIndexPathRequest<'a> {
        GetIndexPathRequest::new(self.client)
    }

    /// Persist the current Assets Index to a file on disk.
    pub fn persist_index_to_file(&self) -> PersistIndexToFileRequest<'a> {
        PersistIndexToFileRequest::new(self.client)
    }

    /// Start an asynchronous reindex of the Assets Index for the current node.
    pub fn start_reindex_current_node(&self) -> StartReindexCurrentNodeRequest<'a> {
        StartReindexCurrentNodeRequest::new(self.client)
    }

    /// Start an asynchronous reindex of the Assets Index for the entire cluster.
    pub fn start_reindex_insight(&self) -> StartReindexInsightRequest<'a> {
        StartReindexInsightRequest::new(self.client)
    }
}

/// Check the integrity of the index.
#[derive(Clone)]
pub struct CheckIndexIntegrityRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CheckIndexIntegrityRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/assets/1.0/index/checkNodeIntegrity".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexIntegrityOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Get the path to the current Assets Index.
#[derive(Clone)]
pub struct GetIndexPathRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIndexPathRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/assets/1.0/index/path".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexPath> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Persist the current Assets Index to a file on disk.
#[derive(Clone)]
pub struct PersistIndexToFileRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PersistIndexToFileRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/assets/1.0/index/persist".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IndexPersistResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Start an asynchronous reindex of the Assets Index for the current node.
#[derive(Clone)]
pub struct StartReindexCurrentNodeRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> StartReindexCurrentNodeRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/index/reindex/currentnode".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Start an asynchronous reindex of the Assets Index for the entire cluster.
#[derive(Clone)]
pub struct StartReindexInsightRequest<'a> {
    client: &'a crate::core::Client,
    clean: Option<String>,
}

impl<'a> StartReindexInsightRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, clean: None }
    }

    /// If true, the index will be cleaned before the reindex starts.
    #[must_use]
    pub fn clean(mut self, value: impl Into<String>) -> Self {
        self.clean = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/assets/1.0/index/reindex/start".to_owned(),
        );

        if let Some(value) = &self.clean {
            config.query.push(("clean".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProgressOut> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

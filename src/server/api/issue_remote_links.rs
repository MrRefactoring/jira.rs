// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueRemoteLinks operations.
pub struct IssueRemoteLinksService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueRemoteLinksService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get remote issue links for an issue.
    pub fn get_remote_issue_links(&self, issue_id_or_key: impl Into<String>) -> GetRemoteIssueLinksRequest<'a> {
        GetRemoteIssueLinksRequest::new(self.client, issue_id_or_key)
    }

    /// Creates or updates a remote issue link from a JSON representation. If a globalId is provided and a remote issue link exists with that globalId, the remote issue link is updated. Otherwise, the remote issue link is created.
    pub fn create_or_update_remote_issue_link(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> CreateOrUpdateRemoteIssueLinkRequest<'a> {
        CreateOrUpdateRemoteIssueLinkRequest::new(self.client, issue_id_or_key)
    }

    /// Delete the remote issue link with the given global id on the issue.
    pub fn delete_remote_issue_link_by_global_id(
        &self,
        issue_id_or_key: impl Into<String>,
        global_id: impl Into<String>,
    ) -> DeleteRemoteIssueLinkByGlobalIdRequest<'a> {
        DeleteRemoteIssueLinkByGlobalIdRequest::new(self.client, issue_id_or_key, global_id)
    }

    /// Get a remote issue link by its id.
    pub fn get_remote_issue_link_by_id(
        &self,
        link_id: impl Into<String>,
        issue_id_or_key: impl Into<String>,
    ) -> GetRemoteIssueLinkByIdRequest<'a> {
        GetRemoteIssueLinkByIdRequest::new(self.client, link_id, issue_id_or_key)
    }

    /// Updates a remote issue link from a JSON representation. Any fields not provided are set to null.
    pub fn update_remote_issue_link(
        &self,
        link_id: impl Into<String>,
        issue_id_or_key: impl Into<String>,
    ) -> UpdateRemoteIssueLinkRequest<'a> {
        UpdateRemoteIssueLinkRequest::new(self.client, link_id, issue_id_or_key)
    }

    /// Delete the remote issue link with the given id on the issue.
    pub fn delete_remote_issue_link_by_id(
        &self,
        link_id: impl Into<String>,
        issue_id_or_key: impl Into<String>,
    ) -> DeleteRemoteIssueLinkByIdRequest<'a> {
        DeleteRemoteIssueLinkByIdRequest::new(self.client, link_id, issue_id_or_key)
    }
}

/// Get remote issue links for an issue.
#[derive(Clone)]
pub struct GetRemoteIssueLinksRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    global_id: Option<String>,
}

impl<'a> GetRemoteIssueLinksRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), global_id: None }
    }

    /// Global id of the remote issue link
    #[must_use]
    pub fn global_id(mut self, value: impl Into<String>) -> Self {
        self.global_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/remotelink", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.global_id {
            config.query.push(("globalId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<RemoteIssueLink>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates or updates a remote issue link from a JSON representation. If a globalId is provided and a remote issue link exists with that globalId, the remote issue link is updated. Otherwise, the remote issue link is created.
#[derive(Clone)]
pub struct CreateOrUpdateRemoteIssueLinkRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    remote_issue_link_create_or_update_request: Option<RemoteIssueLinkCreateOrUpdateRequest>,
}

impl<'a> CreateOrUpdateRemoteIssueLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), remote_issue_link_create_or_update_request: None }
    }

    #[must_use]
    pub fn remote_issue_link_create_or_update_request(mut self, value: RemoteIssueLinkCreateOrUpdateRequest) -> Self {
        self.remote_issue_link_create_or_update_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/remotelink", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        let body = match serde_json::to_value(&self.remote_issue_link_create_or_update_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteIssueLink> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete the remote issue link with the given global id on the issue.
#[derive(Clone)]
pub struct DeleteRemoteIssueLinkByGlobalIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    global_id: String,
}

impl<'a> DeleteRemoteIssueLinkByGlobalIdRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>, global_id: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), global_id: global_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issue/{}/remotelink", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        config.query.push(("globalId".to_owned(), crate::core::QueryValue::Scalar(self.global_id.clone())));

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

/// Get a remote issue link by its id.
#[derive(Clone)]
pub struct GetRemoteIssueLinkByIdRequest<'a> {
    client: &'a crate::core::Client,
    link_id: String,
    issue_id_or_key: String,
}

impl<'a> GetRemoteIssueLinkByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, link_id: link_id.into(), issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/2/issue/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.link_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RemoteIssueLink> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a remote issue link from a JSON representation. Any fields not provided are set to null.
#[derive(Clone)]
pub struct UpdateRemoteIssueLinkRequest<'a> {
    client: &'a crate::core::Client,
    link_id: String,
    issue_id_or_key: String,
    remote_issue_link_create_or_update_request: Option<RemoteIssueLinkCreateOrUpdateRequest>,
}

impl<'a> UpdateRemoteIssueLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> Self {
        Self {
            client,
            link_id: link_id.into(),
            issue_id_or_key: issue_id_or_key.into(),
            remote_issue_link_create_or_update_request: None,
        }
    }

    #[must_use]
    pub fn remote_issue_link_create_or_update_request(mut self, value: RemoteIssueLinkCreateOrUpdateRequest) -> Self {
        self.remote_issue_link_create_or_update_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/issue/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.link_id)
            ),
        );

        let body = match serde_json::to_value(&self.remote_issue_link_create_or_update_request)? {
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

/// Delete the remote issue link with the given id on the issue.
#[derive(Clone)]
pub struct DeleteRemoteIssueLinkByIdRequest<'a> {
    client: &'a crate::core::Client,
    link_id: String,
    issue_id_or_key: String,
}

impl<'a> DeleteRemoteIssueLinkByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, link_id: impl Into<String>, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, link_id: link_id.into(), issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/issue/{}/remotelink/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.link_id)
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

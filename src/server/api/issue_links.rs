// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueLinks operations.
pub struct IssueLinksService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueLinksService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates an issue link between two issues.
    pub fn link_issues(&self, link_issue_request_json: LinkIssueRequestJson) -> LinkIssuesRequest<'a> {
        LinkIssuesRequest::new(self.client, link_issue_request_json)
    }

    /// Returns an issue link with the specified id.
    pub fn get_issue_link(&self, link_id: impl Into<String>) -> GetIssueLinkRequest<'a> {
        GetIssueLinkRequest::new(self.client, link_id)
    }

    /// Deletes an issue link with the specified id.
    pub fn delete_issue_link(&self, link_id: impl Into<String>) -> DeleteIssueLinkRequest<'a> {
        DeleteIssueLinkRequest::new(self.client, link_id)
    }
}

/// Creates an issue link between two issues.
#[derive(Clone)]
pub struct LinkIssuesRequest<'a> {
    client: &'a crate::core::Client,
    link_issue_request_json: LinkIssueRequestJson,
}

impl<'a> LinkIssuesRequest<'a> {
    fn new(client: &'a crate::core::Client, link_issue_request_json: LinkIssueRequestJson) -> Self {
        Self { client, link_issue_request_json }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/issueLink".to_owned());

        let body = match serde_json::to_value(&self.link_issue_request_json)? {
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

/// Returns an issue link with the specified id.
#[derive(Clone)]
pub struct GetIssueLinkRequest<'a> {
    client: &'a crate::core::Client,
    link_id: String,
}

impl<'a> GetIssueLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, link_id: impl Into<String>) -> Self {
        Self { client, link_id: link_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issueLink/{}", crate::core::encode_path_segment(&self.link_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLink> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an issue link with the specified id.
#[derive(Clone)]
pub struct DeleteIssueLinkRequest<'a> {
    client: &'a crate::core::Client,
    link_id: String,
}

impl<'a> DeleteIssueLinkRequest<'a> {
    fn new(client: &'a crate::core::Client, link_id: impl Into<String>) -> Self {
        Self { client, link_id: link_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issueLink/{}", crate::core::encode_path_segment(&self.link_id)),
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

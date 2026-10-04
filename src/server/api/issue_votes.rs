// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueVotes operations.
pub struct IssueVotesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueVotesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// A REST sub-resource representing the voters on the issue.
    pub fn get_votes(&self, issue_id_or_key: impl Into<String>) -> GetVotesRequest<'a> {
        GetVotesRequest::new(self.client, issue_id_or_key)
    }

    /// Adds voter (currently logged user) to particular ticket. You need to be logged in to use this method.
    pub fn add_vote(&self, issue_id_or_key: impl Into<String>) -> AddVoteRequest<'a> {
        AddVoteRequest::new(self.client, issue_id_or_key)
    }

    /// Remove your vote from an issue.
    pub fn remove_vote(&self, issue_id_or_key: impl Into<String>) -> RemoveVoteRequest<'a> {
        RemoveVoteRequest::new(self.client, issue_id_or_key)
    }
}

/// A REST sub-resource representing the voters on the issue.
#[derive(Clone)]
pub struct GetVotesRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetVotesRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/votes", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vote> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds voter (currently logged user) to particular ticket. You need to be logged in to use this method.
#[derive(Clone)]
pub struct AddVoteRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> AddVoteRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/votes", crate::core::encode_path_segment(&self.issue_id_or_key)),
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

/// Remove your vote from an issue.
#[derive(Clone)]
pub struct RemoveVoteRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> RemoveVoteRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issue/{}/votes", crate::core::encode_path_segment(&self.issue_id_or_key)),
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

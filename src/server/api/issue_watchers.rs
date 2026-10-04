// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueWatchers operations.
pub struct IssueWatchersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueWatchersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the list of watchers for the issue with the given key.
    pub fn get_issue_watchers(&self, issue_id_or_key: impl Into<String>) -> GetIssueWatchersRequest<'a> {
        GetIssueWatchersRequest::new(self.client, issue_id_or_key)
    }

    /// Adds a user to an issue's watcher list.
    pub fn add_watcher(&self, issue_id_or_key: impl Into<String>) -> AddWatcherRequest<'a> {
        AddWatcherRequest::new(self.client, issue_id_or_key)
    }

    /// Removes a user from an issue's watcher list.
    pub fn remove_watcher(&self, issue_id_or_key: impl Into<String>) -> RemoveWatcherRequest<'a> {
        RemoveWatcherRequest::new(self.client, issue_id_or_key)
    }
}

/// Returns the list of watchers for the issue with the given key.
#[derive(Clone)]
pub struct GetIssueWatchersRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
}

impl<'a> GetIssueWatchersRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issue/{}/watchers", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Watchers> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a user to an issue's watcher list.
#[derive(Clone)]
pub struct AddWatcherRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    user_name: Option<String>,
    body: Option<String>,
}

impl<'a> AddWatcherRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), user_name: None, body: None }
    }

    /// The name of the user to add to the watcher list. If no name is specified, the current user is added.
    #[must_use]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());

        self
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/issue/{}/watchers", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.user_name {
            config.query.push(("userName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Removes a user from an issue's watcher list.
#[derive(Clone)]
pub struct RemoveWatcherRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    user_name: Option<String>,
    username: Option<String>,
}

impl<'a> RemoveWatcherRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), user_name: None, username: None }
    }

    /// The name of the user to remove from the watcher list.
    #[must_use]
    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());

        self
    }

    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/issue/{}/watchers", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        if let Some(value) = &self.user_name {
            config.query.push(("userName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

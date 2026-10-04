// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ScreenTabs operations.
pub struct ScreenTabsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ScreenTabsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all tabs for the given screen.
    pub fn get_all_screen_tabs(&self, screen_id: i64) -> GetAllScreenTabsRequest<'a> {
        GetAllScreenTabsRequest::new(self.client, screen_id)
    }

    /// Creates tab for given screen.
    pub fn add_screen_tab(&self, screen_id: i64) -> AddScreenTabRequest<'a> {
        AddScreenTabRequest::new(self.client, screen_id)
    }

    /// Renames tab on given screen.
    pub fn rename_screen_tab(&self, tab_id: i64, screen_id: i64) -> RenameScreenTabRequest<'a> {
        RenameScreenTabRequest::new(self.client, tab_id, screen_id)
    }

    /// Deletes tab from given screen.
    pub fn delete_screen_tab(&self, tab_id: i64, screen_id: i64) -> DeleteScreenTabRequest<'a> {
        DeleteScreenTabRequest::new(self.client, tab_id, screen_id)
    }

    /// Moves tab position.
    pub fn move_screen_tab(&self, tab_id: i64, screen_id: i64, pos: i64) -> MoveScreenTabRequest<'a> {
        MoveScreenTabRequest::new(self.client, tab_id, screen_id, pos)
    }
}

/// Returns a list of all tabs for the given screen.
#[derive(Clone)]
pub struct GetAllScreenTabsRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    project_key: Option<String>,
}

impl<'a> GetAllScreenTabsRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64) -> Self {
        Self { client, screen_id, project_key: None }
    }

    /// the key of the project; this parameter is optional
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/screens/{}/tabs", self.screen_id),
        );

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ScreenableTab>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates tab for given screen.
#[derive(Clone)]
pub struct AddScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    screenable_tab: Option<ScreenableTab>,
}

impl<'a> AddScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64) -> Self {
        Self { client, screen_id, screenable_tab: None }
    }

    #[must_use]
    pub fn screenable_tab(mut self, value: ScreenableTab) -> Self {
        self.screenable_tab = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/screens/{}/tabs", self.screen_id),
        );

        let body = match serde_json::to_value(&self.screenable_tab)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScreenableTab> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Renames tab on given screen.
#[derive(Clone)]
pub struct RenameScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    screenable_tab: Option<ScreenableTab>,
}

impl<'a> RenameScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64) -> Self {
        Self { client, tab_id, screen_id, screenable_tab: None }
    }

    #[must_use]
    pub fn screenable_tab(mut self, value: ScreenableTab) -> Self {
        self.screenable_tab = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/screens/{}/tabs/{}", self.screen_id, self.tab_id),
        );

        let body = match serde_json::to_value(&self.screenable_tab)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScreenableTab> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes tab from given screen.
#[derive(Clone)]
pub struct DeleteScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
}

impl<'a> DeleteScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64) -> Self {
        Self { client, tab_id, screen_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/screens/{}/tabs/{}", self.screen_id, self.tab_id),
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

/// Moves tab position.
#[derive(Clone)]
pub struct MoveScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    pos: i64,
}

impl<'a> MoveScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64, pos: i64) -> Self {
        Self { client, tab_id, screen_id, pos }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/screens/{}/tabs/{}/move/{}", self.screen_id, self.tab_id, self.pos),
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

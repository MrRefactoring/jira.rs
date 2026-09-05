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

    /// Returns the list of tabs for a screen.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    ///  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.
    pub fn get_all_screen_tabs(&self, screen_id: i64) -> GetAllScreenTabsRequest<'a> {
        GetAllScreenTabsRequest::new(self.client, screen_id)
    }

    /// Creates a tab for a screen.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn add_screen_tab(&self, screen_id: i64, screenable_tab: ScreenableTab) -> AddScreenTabRequest<'a> {
        AddScreenTabRequest::new(self.client, screen_id, screenable_tab)
    }

    /// Updates the name of a screen tab.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn rename_screen_tab(
        &self,
        screen_id: i64,
        tab_id: i64,
        screenable_tab: ScreenableTab,
    ) -> RenameScreenTabRequest<'a> {
        RenameScreenTabRequest::new(self.client, screen_id, tab_id, screenable_tab)
    }

    /// Deletes a screen tab.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_screen_tab(&self, screen_id: i64, tab_id: i64) -> DeleteScreenTabRequest<'a> {
        DeleteScreenTabRequest::new(self.client, screen_id, tab_id)
    }

    /// Moves a screen tab.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn move_screen_tab(&self, screen_id: i64, tab_id: i64, pos: i64) -> MoveScreenTabRequest<'a> {
        MoveScreenTabRequest::new(self.client, screen_id, tab_id, pos)
    }
}

/// Returns the list of tabs for a screen.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
///  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.
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

    /// The key of the project.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/screens/{}/tabs", self.screen_id),
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

/// Creates a tab for a screen.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct AddScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    screenable_tab: ScreenableTab,
}

impl<'a> AddScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64, screenable_tab: ScreenableTab) -> Self {
        Self { client, screen_id, screenable_tab }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/screens/{}/tabs", self.screen_id),
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

/// Updates the name of a screen tab.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct RenameScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    tab_id: i64,
    screenable_tab: ScreenableTab,
}

impl<'a> RenameScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64, tab_id: i64, screenable_tab: ScreenableTab) -> Self {
        Self { client, screen_id, tab_id, screenable_tab }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/screens/{}/tabs/{}", self.screen_id, self.tab_id),
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

/// Deletes a screen tab.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct DeleteScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    tab_id: i64,
}

impl<'a> DeleteScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64, tab_id: i64) -> Self {
        Self { client, screen_id, tab_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/screens/{}/tabs/{}", self.screen_id, self.tab_id),
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

/// Moves a screen tab.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct MoveScreenTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    tab_id: i64,
    pos: i64,
}

impl<'a> MoveScreenTabRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64, tab_id: i64, pos: i64) -> Self {
        Self { client, screen_id, tab_id, pos }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/screens/{}/tabs/{}/move/{}", self.screen_id, self.tab_id, self.pos),
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

// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Screens operations.
pub struct ScreensService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ScreensService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Adds field or custom field to the default tab.
    pub fn get_all_screens(&self) -> GetAllScreensRequest<'a> {
        GetAllScreensRequest::new(self.client)
    }

    /// Moves field on the given tab.
    pub fn add_field_to_default_screen(&self, field_id: impl Into<String>) -> AddFieldToDefaultScreenRequest<'a> {
        AddFieldToDefaultScreenRequest::new(self.client, field_id)
    }

    /// Gets available fields for screen. i.e ones that haven't already been added.
    pub fn get_fields_to_add(&self, screen_id: i64) -> GetFieldsToAddRequest<'a> {
        GetFieldsToAddRequest::new(self.client, screen_id)
    }

    /// Returns a list of all tabs for the given screen.
    pub fn get_all_tabs(&self, screen_id: i64) -> GetAllTabsRequest<'a> {
        GetAllTabsRequest::new(self.client, screen_id)
    }

    /// Creates tab for given screen.
    pub fn add_tab(&self, screen_id: i64) -> AddTabRequest<'a> {
        AddTabRequest::new(self.client, screen_id)
    }

    /// Renames tab on given screen.
    pub fn rename_tab(&self, tab_id: i64, screen_id: i64) -> RenameTabRequest<'a> {
        RenameTabRequest::new(self.client, tab_id, screen_id)
    }

    /// Deletes tab from given screen.
    pub fn delete_tab(&self, tab_id: i64, screen_id: i64) -> DeleteTabRequest<'a> {
        DeleteTabRequest::new(self.client, tab_id, screen_id)
    }

    /// Gets all fields for a given tab.
    pub fn get_all_fields(&self, tab_id: i64, screen_id: i64) -> GetAllFieldsRequest<'a> {
        GetAllFieldsRequest::new(self.client, tab_id, screen_id)
    }

    /// Adds field to the given tab.
    pub fn add_field(&self, tab_id: i64, screen_id: i64) -> AddFieldRequest<'a> {
        AddFieldRequest::new(self.client, tab_id, screen_id)
    }

    /// Removes field from given tab.
    pub fn remove_field(&self, tab_id: i64, screen_id: i64, id: impl Into<String>) -> RemoveFieldRequest<'a> {
        RemoveFieldRequest::new(self.client, tab_id, screen_id, id)
    }

    /// Moves field on the given tab.
    pub fn move_field(&self, tab_id: i64, screen_id: i64, id: impl Into<String>) -> MoveFieldRequest<'a> {
        MoveFieldRequest::new(self.client, tab_id, screen_id, id)
    }

    /// Update 'showWhenEmptyIndicator' for given field on screen.
    pub fn update_show_when_empty_indicator(
        &self,
        tab_id: i64,
        screen_id: i64,
        new_value: bool,
        id: impl Into<String>,
    ) -> UpdateShowWhenEmptyIndicatorRequest<'a> {
        UpdateShowWhenEmptyIndicatorRequest::new(self.client, tab_id, screen_id, new_value, id)
    }

    /// Moves tab position.
    pub fn move_tab(&self, tab_id: i64, screen_id: i64, pos: i64) -> MoveTabRequest<'a> {
        MoveTabRequest::new(self.client, tab_id, screen_id, pos)
    }
}

/// Adds field or custom field to the default tab.
pub struct GetAllScreensRequest<'a> {
    client: &'a crate::core::Client,
    search: Option<String>,
    expand: Option<String>,
    max_results: Option<String>,
    start_at: Option<String>,
}

impl<'a> GetAllScreensRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, search: None, expand: None, max_results: None, start_at: None }
    }

    #[must_use]
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());

        self
    }

    #[must_use]
    pub fn expand(mut self, value: impl Into<String>) -> Self {
        self.expand = Some(value.into());

        self
    }

    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    #[must_use]
    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/screens".to_owned());

        if let Some(value) = &self.search {
            config.query.push(("search".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Screen>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Moves field on the given tab.
pub struct AddFieldToDefaultScreenRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
}

impl<'a> AddFieldToDefaultScreenRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self { client, field_id: field_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/screens/addToDefault/{}", self.field_id),
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

/// Gets available fields for screen. i.e ones that haven't already been added.
pub struct GetFieldsToAddRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
}

impl<'a> GetFieldsToAddRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_id: i64) -> Self {
        Self { client, screen_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/screens/{}/availableFields", self.screen_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ScreenableField>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of all tabs for the given screen.
pub struct GetAllTabsRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    project_key: Option<String>,
}

impl<'a> GetAllTabsRequest<'a> {
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
pub struct AddTabRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
    screenable_tab: Option<ScreenableTab>,
}

impl<'a> AddTabRequest<'a> {
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
pub struct RenameTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    screenable_tab: Option<ScreenableTab>,
}

impl<'a> RenameTabRequest<'a> {
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
pub struct DeleteTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
}

impl<'a> DeleteTabRequest<'a> {
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

/// Gets all fields for a given tab.
pub struct GetAllFieldsRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    project_key: Option<String>,
}

impl<'a> GetAllFieldsRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64) -> Self {
        Self { client, tab_id, screen_id, project_key: None }
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
            format!("/rest/api/2/screens/{}/tabs/{}/fields", self.screen_id, self.tab_id),
        );

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ScreenableField>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds field to the given tab.
pub struct AddFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    add_field: Option<AddField>,
}

impl<'a> AddFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64) -> Self {
        Self { client, tab_id, screen_id, add_field: None }
    }

    #[must_use]
    pub fn add_field(mut self, value: AddField) -> Self {
        self.add_field = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/screens/{}/tabs/{}/fields", self.screen_id, self.tab_id),
        );

        let body = match serde_json::to_value(&self.add_field)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScreenableField> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes field from given tab.
pub struct RemoveFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    id: String,
}

impl<'a> RemoveFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64, id: impl Into<String>) -> Self {
        Self { client, tab_id, screen_id, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/screens/{}/tabs/{}/fields/{}", self.screen_id, self.tab_id, self.id),
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

/// Moves field on the given tab.
pub struct MoveFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    id: String,
    move_field: Option<MoveField>,
}

impl<'a> MoveFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64, id: impl Into<String>) -> Self {
        Self { client, tab_id, screen_id, id: id.into(), move_field: None }
    }

    #[must_use]
    pub fn move_field(mut self, value: MoveField) -> Self {
        self.move_field = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/2/screens/{}/tabs/{}/fields/{}/move", self.screen_id, self.tab_id, self.id),
        );

        let body = match serde_json::to_value(&self.move_field)? {
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

/// Update 'showWhenEmptyIndicator' for given field on screen.
pub struct UpdateShowWhenEmptyIndicatorRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    new_value: bool,
    id: String,
}

impl<'a> UpdateShowWhenEmptyIndicatorRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        tab_id: i64,
        screen_id: i64,
        new_value: bool,
        id: impl Into<String>,
    ) -> Self {
        Self { client, tab_id, screen_id, new_value, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/2/screens/{}/tabs/{}/fields/{}/updateShowWhenEmptyIndicator/{}",
                self.screen_id, self.tab_id, self.id, self.new_value
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

/// Moves tab position.
pub struct MoveTabRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    pos: i64,
}

impl<'a> MoveTabRequest<'a> {
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

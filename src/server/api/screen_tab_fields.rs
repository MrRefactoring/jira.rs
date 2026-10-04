// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ScreenTabFields operations.
pub struct ScreenTabFieldsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ScreenTabFieldsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Gets all fields for a given tab.
    pub fn get_all_screen_tab_fields(&self, tab_id: i64, screen_id: i64) -> GetAllScreenTabFieldsRequest<'a> {
        GetAllScreenTabFieldsRequest::new(self.client, tab_id, screen_id)
    }

    /// Adds field to the given tab.
    pub fn add_screen_tab_field(&self, tab_id: i64, screen_id: i64) -> AddScreenTabFieldRequest<'a> {
        AddScreenTabFieldRequest::new(self.client, tab_id, screen_id)
    }

    /// Removes field from given tab.
    pub fn remove_screen_tab_field(
        &self,
        tab_id: i64,
        screen_id: i64,
        id: impl Into<String>,
    ) -> RemoveScreenTabFieldRequest<'a> {
        RemoveScreenTabFieldRequest::new(self.client, tab_id, screen_id, id)
    }

    /// Moves field on the given tab.
    pub fn move_screen_tab_field(
        &self,
        tab_id: i64,
        screen_id: i64,
        id: impl Into<String>,
    ) -> MoveScreenTabFieldRequest<'a> {
        MoveScreenTabFieldRequest::new(self.client, tab_id, screen_id, id)
    }
}

/// Gets all fields for a given tab.
#[derive(Clone)]
pub struct GetAllScreenTabFieldsRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    project_key: Option<String>,
}

impl<'a> GetAllScreenTabFieldsRequest<'a> {
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
#[derive(Clone)]
pub struct AddScreenTabFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    add_field: Option<AddField>,
}

impl<'a> AddScreenTabFieldRequest<'a> {
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
#[derive(Clone)]
pub struct RemoveScreenTabFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    id: String,
}

impl<'a> RemoveScreenTabFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, tab_id: i64, screen_id: i64, id: impl Into<String>) -> Self {
        Self { client, tab_id, screen_id, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/2/screens/{}/tabs/{}/fields/{}",
                self.screen_id,
                self.tab_id,
                crate::core::encode_path_segment(&self.id)
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

/// Moves field on the given tab.
#[derive(Clone)]
pub struct MoveScreenTabFieldRequest<'a> {
    client: &'a crate::core::Client,
    tab_id: i64,
    screen_id: i64,
    id: String,
    move_field: Option<MoveField>,
}

impl<'a> MoveScreenTabFieldRequest<'a> {
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
            format!(
                "/rest/api/2/screens/{}/tabs/{}/fields/{}/move",
                self.screen_id,
                self.tab_id,
                crate::core::encode_path_segment(&self.id)
            ),
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

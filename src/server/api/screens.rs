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
    pub fn get_screens(&self) -> GetScreensRequest<'a> {
        GetScreensRequest::new(self.client)
    }

    /// Moves field on the given tab.
    pub fn add_field_to_default_screen(&self, field_id: impl Into<String>) -> AddFieldToDefaultScreenRequest<'a> {
        AddFieldToDefaultScreenRequest::new(self.client, field_id)
    }

    /// Gets available fields for screen. i.e ones that haven't already been added.
    pub fn get_available_screen_fields(&self, screen_id: i64) -> GetAvailableScreenFieldsRequest<'a> {
        GetAvailableScreenFieldsRequest::new(self.client, screen_id)
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
}

/// Adds field or custom field to the default tab.
#[derive(Clone)]
pub struct GetScreensRequest<'a> {
    client: &'a crate::core::Client,
    search: Option<String>,
    expand: Option<String>,
    max_results: Option<i64>,
    start_at: Option<i64>,
}

impl<'a> GetScreensRequest<'a> {
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
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

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
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Screen>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.clone().send().await?;
            let items = page;
            Ok(crate::core::PageStep::offset(items, position, false, None, request.max_results))
        })
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
#[derive(Clone)]
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
            format!("/rest/api/2/screens/addToDefault/{}", crate::core::encode_path_segment(&self.field_id)),
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
#[derive(Clone)]
pub struct GetAvailableScreenFieldsRequest<'a> {
    client: &'a crate::core::Client,
    screen_id: i64,
}

impl<'a> GetAvailableScreenFieldsRequest<'a> {
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

/// Update 'showWhenEmptyIndicator' for given field on screen.
#[derive(Clone)]
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
                self.screen_id,
                self.tab_id,
                crate::core::encode_path_segment(&self.id),
                self.new_value
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

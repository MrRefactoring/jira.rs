// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetScreensForFieldRequestExpandValue {
        Tab => "tab",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about screens in the response. This parameter accepts `tab` which returns details about the screen tabs the field is used in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetScreensForFieldRequestExpand {
    One(GetScreensForFieldRequestExpandValue),
    Many(Vec<GetScreensForFieldRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    pub enum GetScreensRequestScope {
        Global => "GLOBAL",
        Template => "TEMPLATE",
        Project => "PROJECT",
    }
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `id` Sorts by screen ID.
    ///  *  `name` Sorts by screen name.
    pub enum GetScreensRequestOrderBy {
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
        Id => "id",
        IdDescending => "-id",
        IdAscending => "+id",
    }
}

/// The Screens operations.
pub struct ScreensService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ScreensService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of the screens a field is used in.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_screens_for_field(&self, field_id: impl Into<String>) -> GetScreensForFieldRequest<'a> {
        GetScreensForFieldRequest::new(self.client, field_id)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all screens or those specified by one or more screen IDs.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_screens(&self) -> GetScreensRequest<'a> {
        GetScreensRequest::new(self.client)
    }

    /// Adds a field to the default tab of the default screen.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn add_field_to_default_screen(&self, field_id: impl Into<String>) -> AddFieldToDefaultScreenRequest<'a> {
        AddFieldToDefaultScreenRequest::new(self.client, field_id)
    }

    /// Returns the fields that can be added to a tab on a screen.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_available_screen_fields(&self, screen_id: i64) -> GetAvailableScreenFieldsRequest<'a> {
        GetAvailableScreenFieldsRequest::new(self.client, screen_id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of the screens a field is used in.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetScreensForFieldRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
    expand: Option<GetScreensForFieldRequestExpand>,
}

impl<'a> GetScreensForFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self { client, field_id: field_id.into(), start_at: None, max_results: None, expand: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information about screens in the response. This parameter accepts `tab` which returns details about the screen tabs the field is used in.
    #[must_use]
    pub fn expand(mut self, value: GetScreensForFieldRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/field/{}/screens", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ScreenWithTab>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all screens or those specified by one or more screen IDs.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetScreensRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    id: Option<Vec<i64>>,
    query_string: Option<String>,
    scope: Option<Vec<GetScreensRequestScope>>,
    order_by: Option<GetScreensRequestOrderBy>,
}

impl<'a> GetScreensRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, id: None, query_string: None, scope: None, order_by: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The list of screen IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`.
    #[must_use]
    pub fn id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.id = Some(value.into_iter().collect());

        self
    }

    /// String used to perform a case-insensitive partial match with screen name.
    #[must_use]
    pub fn query_string(mut self, value: impl Into<String>) -> Self {
        self.query_string = Some(value.into());

        self
    }

    /// The scope filter string. To filter by multiple scope, provide an ampersand-separated list. For example, `scope=GLOBAL&scope=PROJECT`.
    #[must_use]
    pub fn scope(mut self, value: impl IntoIterator<Item = impl Into<GetScreensRequestScope>>) -> Self {
        self.scope = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `id` Sorts by screen ID.
    ///  *  `name` Sorts by screen name.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetScreensRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/screens".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.id {
            config.query.push(("id".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.query_string {
            config.query.push(("queryString".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.scope {
            config.query.push(("scope".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Screen>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a field to the default tab of the default screen.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
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
            format!("/rest/api/3/screens/addToDefault/{}", crate::core::encode_path_segment(&self.field_id)),
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

/// Returns the fields that can be added to a tab on a screen.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
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
            format!("/rest/api/3/screens/{}/availableFields", self.screen_id),
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

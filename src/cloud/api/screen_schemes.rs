// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetScreenSchemesRequestExpandValue {
        IssueTypeScreenSchemes => "issueTypeScreenSchemes",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) include additional information in the response. This parameter accepts `issueTypeScreenSchemes` that, for each screen schemes, returns information about the issue type screen scheme the screen scheme is assigned to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetScreenSchemesRequestExpand {
    One(GetScreenSchemesRequestExpandValue),
    Many(Vec<GetScreenSchemesRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `id` Sorts by screen scheme ID.
    ///  *  `name` Sorts by screen scheme name.
    pub enum GetScreenSchemesRequestOrderBy {
        Name => "name",
        NameDescending => "-name",
        NameAscending => "+name",
        Id => "id",
        IdDescending => "-id",
        IdAscending => "+id",
    }
}

/// The ScreenSchemes operations.
pub struct ScreenSchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ScreenSchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of screen schemes.
    ///
    /// Only screen schemes used in classic projects are returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_screen_schemes(&self) -> GetScreenSchemesRequest<'a> {
        GetScreenSchemesRequest::new(self.client)
    }

    /// Creates a screen scheme.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_screen_scheme(&self, screen_scheme_details: ScreenSchemeDetails) -> CreateScreenSchemeRequest<'a> {
        CreateScreenSchemeRequest::new(self.client, screen_scheme_details)
    }

    /// Updates a screen scheme. Only screen schemes used in classic projects can be updated.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn update_screen_scheme(
        &self,
        screen_scheme_id: impl Into<String>,
        update_screen_scheme_details: UpdateScreenSchemeDetails,
    ) -> UpdateScreenSchemeRequest<'a> {
        UpdateScreenSchemeRequest::new(self.client, screen_scheme_id, update_screen_scheme_details)
    }

    /// Deletes a screen scheme. A screen scheme cannot be deleted if it is used in an issue type screen scheme.
    ///
    /// Only screens schemes used in classic projects can be deleted.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_screen_scheme(&self, screen_scheme_id: impl Into<String>) -> DeleteScreenSchemeRequest<'a> {
        DeleteScreenSchemeRequest::new(self.client, screen_scheme_id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of screen schemes.
///
/// Only screen schemes used in classic projects are returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetScreenSchemesRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    id: Option<Vec<i64>>,
    expand: Option<GetScreenSchemesRequestExpand>,
    query_string: Option<String>,
    order_by: Option<GetScreenSchemesRequestOrderBy>,
}

impl<'a> GetScreenSchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None, id: None, expand: None, query_string: None, order_by: None }
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

    /// The list of screen scheme IDs. To include multiple IDs, provide an ampersand-separated list. For example, `id=10000&id=10001`.
    #[must_use]
    pub fn id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.id = Some(value.into_iter().collect());

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) include additional information in the response. This parameter accepts `issueTypeScreenSchemes` that, for each screen schemes, returns information about the issue type screen scheme the screen scheme is assigned to.
    #[must_use]
    pub fn expand(mut self, value: GetScreenSchemesRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// String used to perform a case-insensitive partial match with screen scheme name.
    #[must_use]
    pub fn query_string(mut self, value: impl Into<String>) -> Self {
        self.query_string = Some(value.into());

        self
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `id` Sorts by screen scheme ID.
    ///  *  `name` Sorts by screen scheme name.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetScreenSchemesRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/screenscheme".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.id {
            config.query.push(("id".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.query_string {
            config.query.push(("queryString".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ScreenScheme>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a screen scheme.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct CreateScreenSchemeRequest<'a> {
    client: &'a crate::core::Client,
    screen_scheme_details: ScreenSchemeDetails,
}

impl<'a> CreateScreenSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_scheme_details: ScreenSchemeDetails) -> Self {
        Self { client, screen_scheme_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/screenscheme".to_owned());

        let body = match serde_json::to_value(&self.screen_scheme_details)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ScreenSchemeId> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a screen scheme. Only screen schemes used in classic projects can be updated.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct UpdateScreenSchemeRequest<'a> {
    client: &'a crate::core::Client,
    screen_scheme_id: String,
    update_screen_scheme_details: UpdateScreenSchemeDetails,
}

impl<'a> UpdateScreenSchemeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        screen_scheme_id: impl Into<String>,
        update_screen_scheme_details: UpdateScreenSchemeDetails,
    ) -> Self {
        Self { client, screen_scheme_id: screen_scheme_id.into(), update_screen_scheme_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/screenscheme/{}", crate::core::encode_path_segment(&self.screen_scheme_id)),
        );

        let body = match serde_json::to_value(&self.update_screen_scheme_details)? {
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

/// Deletes a screen scheme. A screen scheme cannot be deleted if it is used in an issue type screen scheme.
///
/// Only screens schemes used in classic projects can be deleted.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct DeleteScreenSchemeRequest<'a> {
    client: &'a crate::core::Client,
    screen_scheme_id: String,
}

impl<'a> DeleteScreenSchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, screen_scheme_id: impl Into<String>) -> Self {
        Self { client, screen_scheme_id: screen_scheme_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/screenscheme/{}", crate::core::encode_path_segment(&self.screen_scheme_id)),
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

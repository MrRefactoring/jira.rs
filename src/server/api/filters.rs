// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(CreateFilterRequestExpand { One, Many, Other });

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFavouriteFiltersRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(GetFavouriteFiltersRequestExpand { One, Many, Other });

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(GetFilterRequestExpand { One, Many, Other });

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UpdateFilterRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

crate::core::untagged::untagged!(UpdateFilterRequestExpand { One, Many, Other });

/// The Filters operations.
pub struct FiltersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FiltersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
    pub fn create_filter(&self) -> CreateFilterRequest<'a> {
        CreateFilterRequest::new(self.client)
    }

    /// Returns the favourite filters of the logged-in user
    pub fn get_favourite_filters(&self) -> GetFavouriteFiltersRequest<'a> {
        GetFavouriteFiltersRequest::new(self.client)
    }

    /// Returns a filter given an id
    pub fn get_filter(&self, id: impl Into<String>) -> GetFilterRequest<'a> {
        GetFilterRequest::new(self.client, id)
    }

    /// Updates an existing filter, and returns its new value. The following properties of a filter can be updated: 'jql', 'name', 'description'. Additionally, administrators can also update the 'owner' field. To get, set or unset 'favourite', use rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead.
    pub fn update_filter(&self, id: impl Into<String>) -> UpdateFilterRequest<'a> {
        UpdateFilterRequest::new(self.client, id)
    }

    /// Delete a filter
    pub fn delete_filter(&self, id: impl Into<String>) -> DeleteFilterRequest<'a> {
        DeleteFilterRequest::new(self.client, id)
    }

    /// Returns the default columns for the given filter. Currently logged in user will be used as the user making such request.
    pub fn get_columns(&self, id: impl Into<String>) -> GetColumnsRequest<'a> {
        GetColumnsRequest::new(self.client, id)
    }

    /// Sets the default columns for the given filter
    pub fn set_columns(&self, id: impl Into<String>) -> SetColumnsRequest<'a> {
        SetColumnsRequest::new(self.client, id)
    }

    /// Resets the columns for the given filter such that the filter no longer has its own column config
    pub fn reset_columns(&self, id: impl Into<String>) -> ResetColumnsRequest<'a> {
        ResetColumnsRequest::new(self.client, id)
    }
}

/// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
#[derive(Clone)]
pub struct CreateFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<CreateFilterRequestExpand>,
    filter: Option<Filter>,
}

impl<'a> CreateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None, filter: None }
    }

    #[must_use]
    pub fn expand(mut self, value: CreateFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn filter(mut self, value: Filter) -> Self {
        self.filter = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/filter".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.filter)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the favourite filters of the logged-in user
#[derive(Clone)]
pub struct GetFavouriteFiltersRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetFavouriteFiltersRequestExpand>,
}

impl<'a> GetFavouriteFiltersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: GetFavouriteFiltersRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/filter/favourite".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Filter>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a filter given an id
#[derive(Clone)]
pub struct GetFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<GetFilterRequestExpand>,
    id: String,
}

impl<'a> GetFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: GetFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/filter/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates an existing filter, and returns its new value. The following properties of a filter can be updated: 'jql', 'name', 'description'. Additionally, administrators can also update the 'owner' field. To get, set or unset 'favourite', use rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead.
#[derive(Clone)]
pub struct UpdateFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<UpdateFilterRequestExpand>,
    id: String,
    body: Option<Filter>,
}

impl<'a> UpdateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), expand: None, body: None }
    }

    #[must_use]
    pub fn expand(mut self, value: UpdateFilterRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    #[must_use]
    pub fn body(mut self, value: Filter) -> Self {
        self.body = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/filter/{}", crate::core::encode_path_segment(&self.id)),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Filter> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a filter
#[derive(Clone)]
pub struct DeleteFilterRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> DeleteFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/filter/{}", crate::core::encode_path_segment(&self.id)),
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

/// Returns the default columns for the given filter. Currently logged in user will be used as the user making such request.
#[derive(Clone)]
pub struct GetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/filter/{}/columns", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnLayout>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default columns for the given filter
#[derive(Clone)]
pub struct SetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    columns: Option<Vec<String>>,
}

impl<'a> SetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into(), columns: None }
    }

    #[must_use]
    pub fn columns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.columns = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/filter/{}/columns", crate::core::encode_path_segment(&self.id)),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.columns {
            body.insert("columns".to_owned(), serde_json::to_value(value)?);
        }

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

/// Resets the columns for the given filter such that the filter no longer has its own column config
#[derive(Clone)]
pub struct ResetColumnsRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> ResetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/filter/{}/columns", crate::core::encode_path_segment(&self.id)),
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

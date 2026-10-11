// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Filters operations.
pub struct FiltersService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> FiltersService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
    pub fn create_filter(&self, filter: Filter) -> CreateFilterRequest<'a> {
        CreateFilterRequest::new(self.client, filter)
    }

    /// Returns the favourite filters of the logged-in user
    pub fn get_favourite_filters(&self) -> GetFavouriteFiltersRequest<'a> {
        GetFavouriteFiltersRequest::new(self.client)
    }

    /// Returns a filter given an id
    pub fn get_filter(&self, id: i64) -> GetFilterRequest<'a> {
        GetFilterRequest::new(self.client, id)
    }

    /// Updates an existing filter, and returns its new value. The following properties of a filter can be updated: 'jql', 'name', 'description'. Additionally, administrators can also update the 'owner' field. To get, set or unset 'favourite', use rest/api/1.0/filters/{id}/favourite with GET, PUT and DELETE methods instead.
    pub fn update_filter(&self, id: i64, body: Filter) -> UpdateFilterRequest<'a> {
        UpdateFilterRequest::new(self.client, id, body)
    }

    /// Delete a filter
    pub fn delete_filter(&self, id: i64) -> DeleteFilterRequest<'a> {
        DeleteFilterRequest::new(self.client, id)
    }

    /// Returns the default columns for the given filter. Currently logged in user will be used as the user making such request.
    pub fn get_columns(&self, id: i64) -> GetColumnsRequest<'a> {
        GetColumnsRequest::new(self.client, id)
    }

    /// Sets the default columns for the given filter
    pub fn set_columns(&self, id: i64, column_request_body: ColumnRequestBody) -> SetColumnsRequest<'a> {
        SetColumnsRequest::new(self.client, id, column_request_body)
    }

    /// Resets the columns for the given filter such that the filter no longer has its own column config
    pub fn reset_columns(&self, id: i64) -> ResetColumnsRequest<'a> {
        ResetColumnsRequest::new(self.client, id)
    }
}

/// Creates a new filter, and returns newly created filter. Currently sets permissions just using the users default sharing permissions
#[derive(Clone)]
pub struct CreateFilterRequest<'a> {
    client: &'a crate::core::Client,
    expand: Option<Vec<String>>,
    filter: Filter,
}

impl<'a> CreateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, filter: Filter) -> Self {
        Self { client, filter, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/filter".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.filter)?));

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
    expand: Option<Vec<String>>,
}

impl<'a> GetFavouriteFiltersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/filter/favourite".to_owned());

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
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
    expand: Option<Vec<String>>,
    id: i64,
}

impl<'a> GetFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/rest/api/2/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
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
    expand: Option<Vec<String>>,
    id: i64,
    body: Filter,
}

impl<'a> UpdateFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, body: Filter) -> Self {
        Self { client, id, body, expand: None }
    }

    #[must_use]
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, format!("/rest/api/2/filter/{}", self.id));

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
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
    id: i64,
}

impl<'a> DeleteFilterRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, format!("/rest/api/2/filter/{}", self.id));

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
    id: i64,
}

impl<'a> GetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/filter/{}/columns", self.id),
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
    id: i64,
    column_request_body: ColumnRequestBody,
}

impl<'a> SetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64, column_request_body: ColumnRequestBody) -> Self {
        Self { client, id, column_request_body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/filter/{}/columns", self.id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.column_request_body)?));

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
    id: i64,
}

impl<'a> ResetColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, id: i64) -> Self {
        Self { client, id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/filter/{}/columns", self.id),
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
